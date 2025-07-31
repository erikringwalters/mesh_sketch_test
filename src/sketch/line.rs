use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::assets::materials::UIMaterials;
use crate::cursor::Cursor;

use super::dot::{SketchDotMeshHandle, spawn_sketch_dot};
use super::{
    // size::LINE_WIDTH,
    sketch::{CurrentSketch, SketchMode},
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
                handle_sketch_line
                    .run_if(in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left))),
                handle_sketch_current_line.run_if(in_state(SketchMode::Line)),
                display_lines,
            )
                .chain(),
        );
    }
}

#[hot]
pub fn handle_sketch_line(
    mut commands: Commands,
    mut sketch_dot_mesh: Res<SketchDotMeshHandle>,
    mut ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
) {
    let start_dot: Entity;
    if current_sketch.dots.is_empty() {
        start_dot = spawn_sketch_dot(
            &mut commands,
            &mut sketch_dot_mesh,
            &mut ui_materials,
            cursor.position,
        );
        current_sketch.dots.push(start_dot);
    } else {
        let len = current_sketch.dots.len();
        start_dot = current_sketch.dots[len - 1];
        current_sketch.dots.clear();
    }
    let end_dot = spawn_sketch_dot(
        &mut commands,
        &mut sketch_dot_mesh,
        &mut ui_materials,
        cursor.position,
    );
    current_sketch.dots.push(end_dot);

    current_sketch.lines.clear();
    let line = spawn_line(&mut commands, start_dot, end_dot);
    current_sketch.lines.push(line);
}

#[hot]
pub fn handle_sketch_current_line(
    cursor: Res<Cursor>,
    current_sketch: ResMut<CurrentSketch>,
    mut dots: Query<&mut Transform>,
) {
    if current_sketch.dots.is_empty() {
        return;
    }
    let len = current_sketch.dots.len();
    if let Ok(mut transform) = dots.get_mut(current_sketch.dots[len - 1]) {
        transform.translation = cursor.position;
    } else {
        println!("Could not find currently sketched dot!");
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
            Color::WHITE,
        );
    }
}
