use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::assets::colors::LINE;
use crate::assets::materials::UIMaterials;
use crate::assets::visibility::MESH_VISIBILITY;
use crate::cursor::Cursor;

use super::dot::{Dot, finalize_dots, spawn_temporary_dot};
use super::size::LINE_MESH_WIDTH;
use super::sketch::{Checked, Selected};
use super::sketch::{Current, SketchMode};

#[derive(Component, Debug, PartialEq)]
pub struct Line {
    pub start: Entity,
    pub end: Entity,
}

#[derive(Resource, Debug)]
pub struct LineMeshHandle(pub Handle<Mesh>);

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        let mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Cylinder::new(LINE_MESH_WIDTH, 1.));
        app.insert_resource(LineMeshHandle(mesh_handle))
            .add_systems(
                Update,
                (
                    finalize_dots.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    finalize_lines.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_sketch_line.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    clear_redundant.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_move_current_line.run_if(in_state(SketchMode::Line)),
                    update_line_transforms,
                    // display_dots,
                    display_lines,
                )
                    .chain(),
            );
    }
}

#[hot]
pub fn handle_sketch_line(
    mut commands: Commands,
    cursor: Res<Cursor>,
    mut current: ResMut<Current>,
    selected: Res<Selected>,
    mut checked: ResMut<Checked>,
    lines: Query<&mut Line>,
) {
    let start_dot: Entity;
    let mut prev_line = Entity::PLACEHOLDER;
    if !current.lines.is_empty() {
        prev_line = current.lines[0];
    }

    let curr_empty = current.dots.is_empty();
    let slct_empty = selected.dots.is_empty();

    // New chain starting with new dot
    if curr_empty && slct_empty {
        start_dot = spawn_temporary_dot(&mut commands, cursor.position);
        current.dots.push(start_dot);
    }
    // Continue chain with new dot
    else if !curr_empty && slct_empty {
        start_dot = *current.dots.last().unwrap();
        current.dots.clear();
    }
    // New chain starting with existing dot
    else if curr_empty && !slct_empty {
        start_dot = selected.dots[0];
        current.dots.clear();
    }
    // Continue chain with existing dot
    else {
        let temp_dot = swap_line_end(selected.dots[0], &mut current, lines);
        start_dot = selected.dots[0];
        current.dots.clear();
        commands.entity(temp_dot).despawn();
    }

    let end_dot = spawn_temporary_dot(&mut commands, cursor.position);

    current.dots.push(end_dot);

    current.lines.clear();
    let line = spawn_line(&mut commands, start_dot, end_dot);
    current.lines.push(line);
    checked.lines.clear();
    checked.lines.push(prev_line);
}

#[hot]
pub fn handle_move_current_line(
    cursor: Res<Cursor>,
    current: ResMut<Current>,
    mut dots: Query<&mut Transform>,
) {
    if current.dots.is_empty() {
        return;
    }
    if let Ok(mut transform) = dots.get_mut(*current.dots.last().unwrap()) {
        transform.translation = cursor.position;
    } else {
        warn!("Could not find currently sketched dot!");
    }
}

#[hot]
fn spawn_line(commands: &mut Commands, start: Entity, end: Entity) -> Entity {
    commands
        .spawn(Line {
            start: start,
            end: end,
        })
        .id()
}

#[hot]
fn display_lines(mut gizmos: Gizmos, lines: Query<&Line>, dots: Query<&Transform>) {
    for line in lines.iter() {
        let Ok(start_position) = dots.get(line.start) else {
            continue;
        };
        let Ok(end_position) = dots.get(line.end) else {
            continue;
        };
        gizmos.line(start_position.translation, end_position.translation, LINE);
    }
}

#[hot]
pub fn handle_dot_hover(hover: Trigger<Pointer<Over>>, mut selected: ResMut<Selected>) {
    selected.dots.clear();
    selected.dots.push(hover.target());
}

#[hot]
pub fn handle_dot_end_hover(_hover: Trigger<Pointer<Out>>, mut selected: ResMut<Selected>) {
    selected.dots.clear();
}

#[hot]
pub fn swap_line_end(
    next: Entity,
    current: &mut ResMut<Current>,
    mut lines: Query<&mut Line>,
) -> Entity {
    let mut prev = *current.dots.last().unwrap();
    if let Ok(mut line) = lines.get_mut(current.lines[0]) {
        prev = line.end;
        line.end = next;
    }
    return prev;
}

#[hot]
pub fn finalize_line(
    commands: &mut Commands,
    line_mesh: &Res<LineMeshHandle>,
    ui_materials: &Res<UIMaterials>,
    line_entity: Entity,
    lines: &mut Query<&mut Line>,
    dots: &mut Query<&Transform>,
) {
    let (start_pos, end_pos) = get_line_ending_positions(line_entity, lines, dots);
    let transform = get_line_mesh_transform(start_pos, end_pos);

    commands.entity(line_entity).insert((
        Mesh3d(line_mesh.0.clone()),
        MeshMaterial3d(ui_materials.line.clone()),
        MESH_VISIBILITY,
        Transform::from(transform),
    ));
}

#[hot]
pub fn get_line_ending_positions(
    line_entity: Entity,
    lines: &mut Query<&mut Line>,
    dots: &mut Query<&Transform>,
) -> (Transform, Transform) {
    let mut transforms = (Transform::default(), Transform::default());
    let Ok(line) = lines.get(line_entity) else {
        return transforms;
    };
    let Ok(start) = dots.get(line.start) else {
        return transforms;
    };
    let Ok(end) = dots.get(line.end) else {
        return transforms;
    };
    transforms = (*start, *end);
    return transforms;
}

#[hot]
pub fn get_line_mesh_transform(start: Transform, end: Transform) -> Transform {
    let a = start.translation;
    let b = end.translation;
    let center = (a + b) / 2.;
    let dir = b - a;
    let quat = Quat::from_rotation_arc(Vec3::Y, dir.normalize_or_zero());

    let scale = Vec3::new(LINE_MESH_WIDTH, dir.length(), LINE_MESH_WIDTH);

    Transform {
        translation: center,
        rotation: quat,
        scale: scale,
    }
}

#[hot]
pub fn update_line_transforms(
    mut lines: Query<(&Line, &mut Transform)>,
    dots: Query<(&Dot, &Transform), Without<Line>>,
) {
    for (line, mut transform) in lines.iter_mut() {
        let Ok(start) = dots.get(line.start) else {
            continue;
        };
        let Ok(end) = dots.get(line.end) else {
            continue;
        };
        // let prev_start = start.0.prev_transform;
        // let prev_end = end.0.prev_transform;

        // // Return if no change in dot transforms
        // if &prev_start == start.1 && &prev_end == end.1 {
        //     return;
        // }

        let mesh_transform = get_line_mesh_transform(*start.1, *end.1);
        *transform = mesh_transform;
    }
}

#[hot]
pub fn finalize_lines(
    mut commands: Commands,
    current: ResMut<Current>,
    line_mesh: Res<LineMeshHandle>,
    ui_materials: Res<UIMaterials>,
    mut lines: Query<&mut Line>,
    mut dots: Query<&Transform>,
) {
    for line in &current.lines {
        finalize_line(
            &mut commands,
            &line_mesh,
            &ui_materials,
            *line,
            &mut lines,
            &mut dots,
        );
    }
}

#[hot]
pub fn clear_redundant(
    mut commands: Commands,
    mut checked: ResMut<Checked>,
    lines: Query<(Entity, &mut Line)>,
) {
    if checked.lines.is_empty() || checked.lines[0] == Entity::PLACEHOLDER {
        return;
    }
    let checked_line = checked.lines[0];
    let Ok((compare_to_entity, compare_to)) = lines.get(checked_line) else {
        return;
    };
    for (line_entity, line) in lines.iter() {
        if compare_to_entity == line_entity {
            continue;
        }
        if compare_to.start == compare_to.end {
            warn!("Clearing single-dot line: {:?}", checked_line);
            checked.lines.clear();
            commands.entity(checked_line).despawn();
            return;
        }

        let same_line = line.start == compare_to.start && line.end == compare_to.end;
        let same_line_reversed = line.start == compare_to.end && line.end == compare_to.start;
        if same_line || same_line_reversed {
            warn!("Clearing redundant line: {:?}", checked_line);
            checked.lines.clear();
            commands.entity(checked_line).despawn();
            return;
        }
    }
}
