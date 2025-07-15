use std::f32::consts::PI;

use bevy::{math::ops::atan, prelude::*, transform};
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::{DotMeshHandle, spawn_dot},
    size::LINE_WIDTH,
    sketch::{CurrentSketch, DEFAULT_POS, LineChain, SketchMode, reset_sketch},
};

#[derive(Component, Debug, Default)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Resource, Debug)]
pub struct LineMeshHandle(pub Handle<Mesh>);

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        let mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Cylinder::new(LINE_WIDTH, 1.));
        app.insert_resource(LineMeshHandle(mesh_handle))
            .add_systems(Update, handle_current_line);
    }
}

#[hot]
pub fn handle_sketch_line(
    commands: &mut Commands,
    dot_mesh: &Res<DotMeshHandle>,
    line_mesh: &Res<LineMeshHandle>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
    mut line_chain: ResMut<LineChain>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_sketch(current_sketch, line_chain);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define start of line
    if current_sketch.position[0] == DEFAULT_POS {
        current_sketch.position[0] = cursor.position;
        let start = current_sketch.position[0];
        current_sketch.lines.push(spawn_line(
            commands,
            line_mesh,
            materials,
            start,
            cursor.position,
        ));
    }
    // Define end of line
    else if current_sketch.position[1] == DEFAULT_POS {
        current_sketch.position[1] = cursor.position;
    }

    let start = current_sketch.position[0];
    let end = current_sketch.position[1];

    // Create line and dots entities if both start and end are defined
    if start != DEFAULT_POS && end != DEFAULT_POS {
        if line_chain.count == 0 {
            spawn_dot(commands, dot_mesh, materials, start);
        }
        spawn_dot(commands, dot_mesh, materials, end);
        current_sketch.lines.clear();
        current_sketch.lines.push(spawn_line(
            commands,
            line_mesh,
            materials,
            end,
            cursor.position,
        ));
        current_sketch.position[0] = end;
        current_sketch.position[1] = DEFAULT_POS;
        line_chain.count += 1;
    }
}

#[hot]
fn spawn_line(
    commands: &mut Commands,
    line_mesh: &Res<LineMeshHandle>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    end: Vec3,
) -> Entity {
    commands
        .spawn((
            Line {
                start: start,
                end: end,
            },
            Mesh3d(line_mesh.0.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(1., 1., 1., 1.),
                unlit: true,
                ..default()
            })),
            Transform::from_translation(start).with_rotation(Quat::from_euler(
                EulerRot::YXZ,
                0.0,
                PI / 2.,
                0.0,
            )),
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ))
        .id()
}

#[hot]
fn handle_current_line(
    current_sketch: ResMut<CurrentSketch>,
    cursor: Res<Cursor>,
    state: Res<State<SketchMode>>,
    mut lines: Query<&mut Transform>,
) {
    if state.get() != &SketchMode::Line || current_sketch.lines.is_empty() {
        return;
    }

    // println!("{:?}", current_sketch.lines[0]);
    if let Ok(mut transform) = lines.get_mut(current_sketch.lines[0]) {
        let a = cursor.position;
        let b = current_sketch.position[0];
        transform.translation = (a + b) * 0.5;
        transform.scale.y = (b - a).length();
        let x = (b.y - a.y) / (b.x - a.x);
        transform.rotation.z = atan(x);
        // println!("{:?}", transform.translation);
    }
}
