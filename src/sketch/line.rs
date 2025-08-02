use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::assets::colors::LINE_COLOR;
use crate::assets::materials::UIMaterials;
use crate::cursor::Cursor;

use super::dot::{DotMeshHandle, finalize_dot, spawn_temporary_dot};
use super::{
    // size::LINE_WIDTH,
    sketch::{Current, SketchMode},
};

#[derive(Component, Debug)]
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
                // select_dot
                //     .run_if(in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left))),
                handle_sketch_line
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
) {
    let start_dot: Entity;
    if current.dots.is_empty() {
        start_dot = spawn_temporary_dot(&mut commands, cursor.position);
        current.dots.push(start_dot);
    } else {
        for dot in &current.dots {
            finalize_dot(&mut commands, &dot_mesh, &ui_materials, *dot);
        }
        let len = current.dots.len();
        start_dot = current.dots[len - 1];
        current.dots.clear();
    };

    let end_dot = spawn_temporary_dot(&mut commands, cursor.position);
    current.dots.push(end_dot);

    current.lines.clear();
    let line = spawn_line(&mut commands, start_dot, end_dot);
    current.lines.push(line);
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
    let len = current.dots.len();
    if let Ok(mut transform) = dots.get_mut(current.dots[len - 1]) {
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

// #[hot]
// pub fn select_dot(cursor: Res<Cursor>, mut selected: ResMut<Selected>) {}
