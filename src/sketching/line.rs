use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::assets::colors::*;
use crate::assets::materials::{UIMaterialProvider, UIMaterials};
use crate::assets::visibility::MESH_VISIBILITY;
use crate::cursor::{Cursor, Picking};
use crate::reload::{ReloadLevel, Reloadable};
use crate::schedule::ScheduleSet;

use super::dot::{Dot, finalize_dots, spawn_temporary_dot};
use super::selection::Selected;
use super::size::LINE_MESH_WIDTH;
use super::sketch::{Checked, Current, Moving, SketchMode};

type DotNotMoving = (With<Dot>, Without<Line>, Without<Moving>);

#[derive(Component, Debug, PartialEq)]
pub struct Line {
    pub start: Entity,
    pub end: Entity,
}

impl UIMaterialProvider for Line {
    fn get_material(ui_materials: &UIMaterials) -> Handle<StandardMaterial> {
        ui_materials.line.clone()
    }
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
                    (
                        finalize_dots,
                        finalize_lines,
                        handle_sketch_line,
                        clear_redundant,
                    )
                        .run_if(
                            in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                        ),
                    handle_move_current_line.run_if(in_state(SketchMode::Line)),
                    // display_dots,
                    display_lines,
                )
                    .chain()
                    .in_set(ScheduleSet::EntityUpdates),
            )
            .add_systems(
                Update,
                ((delete_selected_entities, delete_dependent_lines)
                    .run_if(input_just_pressed(KeyCode::KeyX)),)
                    .chain()
                    .in_set(ScheduleSet::DespawnEntities),
            );
    }
}

pub fn handle_sketch_line(
    mut commands: Commands,
    cursor: Res<Cursor>,
    mut current: ResMut<Current>,
    picking: Res<Picking>,
    mut checked: ResMut<Checked>,
    lines: Query<&mut Line>,
) {
    let start_dot: Entity;
    let mut prev_line = Entity::PLACEHOLDER;
    if !current.lines.is_empty() {
        prev_line = current.lines[0];
    }

    let current_empty = current.dots.is_empty();
    let hover_empty = picking.hovered == Entity::PLACEHOLDER;

    // New chain starting with new dot
    if current_empty && hover_empty {
        start_dot = spawn_temporary_dot(&mut commands, cursor.position);
        current.dots.push(start_dot);
    }
    // Continue chain with new dot
    else if !current_empty && hover_empty {
        start_dot = *current.dots.last().unwrap();
        current.dots.clear();
    }
    // New chain starting with existing dot
    else if current_empty && !hover_empty {
        start_dot = picking.hovered;
        current.dots.clear();
    }
    // Continue chain with existing dot
    else {
        let temp_dot = swap_line_end(picking.hovered, &mut current, lines);
        start_dot = picking.hovered;
        current.dots.clear();
        commands.entity(temp_dot).despawn();
    }

    let end_dot = spawn_temporary_dot(&mut commands, cursor.position);

    current.dots.push(end_dot);

    current.lines.clear();
    let line = spawn_line(&mut commands, start_dot, end_dot);
    // setup_observers(line);
    current.lines.push(line);
    checked.lines.clear();
    checked.lines.push(prev_line);
}

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

fn spawn_line(commands: &mut Commands, start: Entity, end: Entity) -> Entity {
    commands
        .spawn((
            Line { start, end },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ))
        .id()
}

fn display_lines(mut gizmos: Gizmos, lines: Query<&Line>, dots: Query<&Transform>) {
    for line in lines.iter() {
        let Ok(start_position) = dots.get(line.start) else {
            continue;
        };
        let Ok(end_position) = dots.get(line.end) else {
            continue;
        };
        gizmos.line(
            start_position.translation,
            end_position.translation,
            color_from_hex(LINE),
        );
    }
}

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
    prev
}

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
        transform,
    ));
}

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
    transforms
}

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
        scale,
    }
}

pub fn mark_moving_lines(
    mut commands: Commands,
    lines: Query<&mut Line, With<Selected>>,
    mut dots: Query<Entity, DotNotMoving>,
) {
    for line in &lines {
        if let Ok(start) = dots.get_mut(line.start) {
            commands.entity(start).insert(Moving);
        }
        if let Ok(end) = dots.get_mut(line.end) {
            commands.entity(end).insert(Moving);
        }
    }
}

pub fn update_line_mesh_transforms(
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

        let mesh_transform = get_line_mesh_transform(*start.1, *end.1);
        *transform = mesh_transform;
    }
}

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

pub fn clear_redundant(
    mut commands: Commands,
    mut checked: ResMut<Checked>,
    lines: Query<(Entity, &mut Line)>,
) {
    if checked.lines.is_empty() || checked.lines[0] == Entity::PLACEHOLDER {
        return;
    }
    let checked_line = checked.lines[0];
    let Ok((compare_to_entity, compare_to_line)) = lines.get(checked_line) else {
        return;
    };
    for (line_entity, line) in lines.iter() {
        if compare_to_entity == line_entity {
            continue;
        }
        if compare_to_line.start == compare_to_line.end {
            warn!("Clearing single-dot line: {:?}", checked_line);
            checked.lines.clear();
            commands.entity(checked_line).despawn();
            return;
        }

        let same_line = line.start == compare_to_line.start && line.end == compare_to_line.end;
        let same_line_reversed =
            line.start == compare_to_line.end && line.end == compare_to_line.start;
        if same_line || same_line_reversed {
            warn!("Clearing redundant line: {:?}", checked_line);
            checked.lines.clear();
            commands.entity(checked_line).despawn();
            return;
        }
    }
}

pub fn delete_selected_entities(mut commands: Commands, query: Query<Entity, With<Selected>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Delete lines if their start or end have been deleted
pub fn delete_dependent_lines(
    mut commands: Commands,
    lines: Query<(Entity, &Line)>,
    mut dots: Query<&Dot>,
) {
    for (entity, line) in lines.iter() {
        if dots.get_mut(line.start).is_ok() {
        } else {
            commands.entity(entity).despawn();
        }
        if dots.get_mut(line.end).is_ok() {
        } else {
            commands.entity(entity).despawn();
        }
    }
}
