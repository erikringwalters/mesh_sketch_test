use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::assets::materials::UIMaterials;
use crate::cursor::Cursor;

use super::dot::{SketchDotMeshHandle, spawn_sketch_dot};
use super::{
    // size::LINE_WIDTH,
    sketch::{CurrentSketch, DEFAULT_POS, LineChain, SketchMode},
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
                start_line_chain
                    .run_if(in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left))),
                handle_sketch_current_line.run_if(in_state(SketchMode::Line)),
                continue_line_chain
                    .run_if(in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left))),
                display_lines,
            )
                .chain(),
        );
    }
}

#[hot]
pub fn start_line_chain(
    mut commands: Commands,
    mut sketch_dot_mesh: Res<SketchDotMeshHandle>,
    mut ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    if line_chain.count > 0 {
        return;
    }
    if line_chain.count == 0 {
        let dot = spawn_sketch_dot(
            &mut commands,
            &mut sketch_dot_mesh,
            &mut ui_materials,
            cursor.position,
        );
        current_sketch.dots.push(dot);
    }
    let dot = spawn_sketch_dot(
        &mut commands,
        &mut sketch_dot_mesh,
        &mut ui_materials,
        cursor.position,
    );
    current_sketch.dots.push(dot);

    let line = spawn_line(&mut commands, &mut current_sketch);
    current_sketch.lines.push(line);
}

#[hot]
pub fn continue_line_chain(
    mut commands: Commands,
    mut sketch_dot_mesh: Res<SketchDotMeshHandle>,
    mut ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
    mut line_chain: ResMut<LineChain>,
) {
    line_chain.count += 1;
    println!(
        "current_sketch.dots: {:?}\ncurrent_sketch.dots length: {:?}",
        current_sketch.dots,
        current_sketch.dots.len()
    );
    println!("line-chain count: {:?}", line_chain.count);

    if line_chain.count < 2 || current_sketch.dots.len() <= 1 || current_sketch.lines.is_empty() {
        return;
    }

    let len = current_sketch.dots.len();
    let start_dot = current_sketch.dots[len - 1];
    current_sketch.dots.clear();
    println!("dot: {:?}", start_dot);
    current_sketch.dots.push(start_dot);
    let end_dot = spawn_sketch_dot(
        &mut commands,
        &mut sketch_dot_mesh,
        &mut ui_materials,
        cursor.position,
    );
    current_sketch.dots.push(end_dot);
    println!("start dot: {:?}, end dot: {:?}", start_dot, end_dot);

    current_sketch.lines.clear();
    let line = spawn_line(&mut commands, &mut current_sketch);
    current_sketch.lines.push(line);
    println!("lines: {:?}", current_sketch.lines);
    println!("");
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
    if let Ok(mut transform) = dots.get_mut(current_sketch.dots[1]) {
        transform.translation = cursor.position;
    }
}

#[hot]
fn spawn_line(commands: &mut Commands, current_sketch: &mut CurrentSketch) -> Entity {
    commands
        .spawn(Line {
            start: current_sketch.dots[0],
            end: current_sketch.dots[1],
        })
        .id()
}

#[hot]
fn display_lines(
    mut gizmos: Gizmos,
    lines: Query<&Line>,
    dots: Query<&Transform>,
    cursor: Res<Cursor>,
    state: Res<State<SketchMode>>,
    current_sketch: ResMut<CurrentSketch>,
) {
    // Display existing lines
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
    // Display currently drawn line
    if state.get() == &SketchMode::Line && current_sketch.position[0] != DEFAULT_POS {
        gizmos.line(current_sketch.position[0], cursor.position, Color::WHITE);
    }
}
