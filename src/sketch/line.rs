use std::f32::consts::PI;

use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::input::common_conditions::input_just_pressed;
use bevy::{math::ops::atan, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::assets::materials::UIMaterials;
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
                    handle_sketch_dot_start.run_if(
                        in_state(SketchMode::Line).and(input_just_pressed(MouseButton::Left)),
                    ),
                    handle_sketch_dot_end.run_if(
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
pub fn handle_sketch_dot_start(
    commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    current_sketch: ResMut<CurrentSketch>,
    line_chain: ResMut<LineChain>,
) {
    if line_chain.count == 0
        && is_defined(current_sketch.position[0])
        && is_defined(current_sketch.position[1])
    {
        spawn_dot(commands, dot_mesh, ui_materials, current_sketch.position[0]);
    }
}

#[hot]
pub fn handle_sketch_dot_end(
    commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    current_sketch: ResMut<CurrentSketch>,
) {
    if is_defined(current_sketch.position[0]) && is_defined(current_sketch.position[1]) {
        spawn_dot(commands, dot_mesh, ui_materials, current_sketch.position[1]);
    }
}

#[hot]
pub fn handle_sketch_line_start(
    commands: Commands,
    line_mesh: Res<LineMeshHandle>,
    ui_materials: Res<UIMaterials>,
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
        ui_materials,
        start,
        cursor.position,
    ));
}

#[hot]
pub fn handle_sketch_line_end(cursor: Res<Cursor>, mut current_sketch: ResMut<CurrentSketch>) {
    if !is_defined(current_sketch.position[0]) {
        return;
    }
    // TODO: Implement picking system that checks which entity we're hovering over
    // If we're hovering over the drawing plane, use cursor position
    // If we're hovering over a dot, use the dot's position
    // TODO: Make line mesh non-pickable while it's being drawn
    current_sketch.position[1] = cursor.position;
}

#[hot]
fn spawn_line(
    mut commands: Commands,
    line_mesh: Res<LineMeshHandle>,
    ui_materials: Res<UIMaterials>,
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
            MeshMaterial3d(ui_materials.line.clone()),
            Transform::from_translation(start),
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Visibility::Hidden,
        ))
        .observe(update_material_on::<Pointer<Over>>(
            ui_materials.hover.clone(),
        ))
        .observe(update_material_on::<Pointer<Out>>(
            ui_materials.line.clone(),
        ))
        .observe(update_material_on::<Pointer<Pressed>>(
            ui_materials.pressed.clone(),
        ))
        .observe(update_material_on::<Pointer<Released>>(
            ui_materials.hover.clone(),
        ))
        .id()
}

#[hot]
pub fn handle_finalize_sketch_line(
    commands: Commands,
    line_mesh: Res<LineMeshHandle>,
    ui_materials: Res<UIMaterials>,
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
            ui_materials,
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
