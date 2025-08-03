use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::assets::colors::LINE_COLOR;
use crate::assets::materials::UIMaterials;
use crate::cursor::Cursor;

use super::dot::{Dot, DotMeshHandle, finalize_dot, spawn_temporary_dot};
use super::sketch::{Checked, Selected};
use super::{
    // size::LINE_WIDTH,
    sketch::{Current, SketchMode},
};

#[derive(Component, Debug, PartialEq)]
pub struct Line {
    pub start: Entity,
    pub end: Entity,
}

// #[derive(Resource, Debug)]
// pub struct LineMeshHandle(pub Handle<Mesh>);

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        // let mesh_handle = app
        //     .world_mut()
        //     .resource_mut::<Assets<Mesh>>()
        //     .add(Cylinder::new(LINE_WIDTH, 1.));
        // app.insert_resource(LineMeshHandle(mesh_handle))
        app.add_systems(
            Update,
            (
                handle_sketch_line
                    .run_if(in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left))),
                clear_redundant
                    .run_if(in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left))),
                handle_move_current_line.run_if(in_state(SketchMode::Line)),
                display_lines,
            )
                .chain(),
        );
    }
}

#[hot]
pub fn handle_sketch_line(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
    mut current: ResMut<Current>,
    selected: Res<Selected>,
    mut checked: ResMut<Checked>,
    mut dot_query: Query<&mut Dot>,
    lines: Query<&mut Line>,
) {
    let start_dot: Entity;
    let mut prev_line = Entity::PLACEHOLDER;
    if !current.lines.is_empty() {
        prev_line = current.lines[0];
    }

    let curr_empty = current.dots.is_empty();
    let slct_empty = selected.dots.is_empty();

    if selected.dots.is_empty() {
        for dot in &current.dots {
            finalize_dot(
                &mut commands,
                &dot_mesh,
                &ui_materials,
                *dot,
                &mut dot_query,
            );
        }
    }

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
    // TODO: Check if line uses 1 dot for start and end

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
        gizmos.line(
            start_position.translation,
            end_position.translation,
            LINE_COLOR,
        );
    }
}

#[hot]
pub fn handle_dot_hover(hover: Trigger<Pointer<Over>>, mut selected: ResMut<Selected>) {
    selected.dots.clear();
    selected.dots.push(hover.target());
}

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
pub fn clear_redundant(
    mut commands: Commands,
    mut checked: ResMut<Checked>,
    lines: Query<&mut Line>,
) {
    if checked.lines.is_empty() || checked.lines[0] == Entity::PLACEHOLDER {
        return;
    }
    let checked_line = checked.lines[0];
    let Ok(compare_to) = lines.get(checked_line) else {
        return;
    };
    for line in lines.iter() {
        println!("line: {:?}\n compare_to: {:?}", line, compare_to);
        if compare_to == line {
            continue;
        }
        if compare_to.start == compare_to.end {
            warn!("Clearing single-point line: {:?}", checked_line);
            checked.lines.clear();
            commands.entity(checked_line).despawn();
            return;
        }

        if (line.start == compare_to.start || line.start == compare_to.end)
            && (line.end == compare_to.start || line.end == compare_to.end)
        {
            warn!("Clearing redundant line: {:?}", checked_line);
            checked.lines.clear();
            commands.entity(checked_line).despawn();
            println!("");
            return;
        }
    }
}
