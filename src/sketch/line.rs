use std::f32::consts::PI;

use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::{math::ops::atan, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::color::*;
use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::sketch::{is_defined, update_material_on};
use super::{
    dot::{DotMeshHandle, spawn_dot},
    size::LINE_WIDTH,
    sketch::{CurrentSketch, DEFAULT_POS, LineChain, SketchMode},
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
            .add_systems(
                Update,
                (
                    handle_sketch_line_end.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_sketch_line_start.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_transform_current_line.run_if(in_state(SketchMode::Line)),
                    handle_sketch_start_dot.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_sketch_end_dot.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_finalize_sketch_line.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                )
                    .chain(),
            );
    }
}

#[hot]
pub fn handle_sketch_start_dot(
    commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    materials: ResMut<Assets<StandardMaterial>>,
    current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    if line_chain.count == 0
        && is_defined(current_sketch.position[0])
        && is_defined(current_sketch.position[1])
    {
        spawn_dot(commands, dot_mesh, materials, current_sketch.position[0]);
    }
}

#[hot]
pub fn handle_sketch_end_dot(
    commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    materials: ResMut<Assets<StandardMaterial>>,
    current_sketch: ResMut<CurrentSketch>,
) {
    if is_defined(current_sketch.position[0]) && is_defined(current_sketch.position[1]) {
        spawn_dot(commands, dot_mesh, materials, current_sketch.position[1]);
    }
}

#[hot]
pub fn handle_sketch_line_start(
    commands: Commands,
    line_mesh: Res<LineMeshHandle>,
    materials: ResMut<Assets<StandardMaterial>>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
) {
    if is_defined(current_sketch.position[0]) {
        return;
    }
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

#[hot]
pub fn handle_sketch_line_end(cursor: Res<Cursor>, mut current_sketch: ResMut<CurrentSketch>) {
    if !is_defined(current_sketch.position[0]) {
        return;
    }
    current_sketch.position[1] = cursor.position;
}

#[hot]
fn spawn_line(
    mut commands: Commands,
    line_mesh: Res<LineMeshHandle>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    end: Vec3,
) -> Entity {
    let default_matl = materials.add(LINE_COLOR);
    let hover_matl = materials.add(HOVER_COLOR);
    let pressed_matl = materials.add(PRESSED_COLOR);

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
            Transform::from_translation(start),
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Visibility::Hidden,
        ))
        .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
        .observe(update_material_on::<Pointer<Out>>(default_matl.clone()))
        .observe(update_material_on::<Pointer<Pressed>>(pressed_matl.clone()))
        .observe(update_material_on::<Pointer<Released>>(hover_matl.clone()))
        .id()
}

#[hot]
pub fn handle_finalize_sketch_line(
    commands: Commands,
    line_mesh: Res<LineMeshHandle>,
    materials: ResMut<Assets<StandardMaterial>>,
    cursor: Res<Cursor>,
    mut current_sketch: ResMut<CurrentSketch>,
    mut line_chain: ResMut<LineChain>,
    mut lines: Query<&Line>,
) {
    if !is_defined(current_sketch.position[1]) {
        return;
    }
    if let Ok(line) = lines.get_mut(current_sketch.lines[0]) {
        let end = line.end;
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
fn handle_transform_current_line(
    current_sketch: ResMut<CurrentSketch>,
    cursor: Res<Cursor>,
    mut lines: Query<(&mut Line, &mut Transform, &mut Visibility)>,
) {
    if current_sketch.lines.is_empty() {
        return;
    }

    if let Ok((mut line, mut transform, mut visibility)) = lines.get_mut(current_sketch.lines[0]) {
        line.start = current_sketch.position[0];
        line.end = cursor.position;

        let a = line.start;
        let b = line.end;
        let n = b.y - a.y;
        let d = b.x - a.x;
        let angle = if d != 0. { atan(n / d) } else { 0. };
        let rot = Vec3::Z * angle + vec3(0., 0., PI / 2.);

        transform.translation = (a + b) / 2.;
        transform.scale.y = (b - a).length();
        transform.rotation = Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z);

        *visibility = Visibility::Visible;
    }
}
