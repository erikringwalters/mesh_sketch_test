use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::{
    assets::materials::UIMaterials,
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    size::DOT_RADIUS,
    sketch::{SketchMode, update_material_on},
};

#[derive(Component, Debug, Default)]
pub struct Dot;
//{
// pub position: Vec3,
//}

#[derive(Resource, Debug)]
pub struct DotMeshHandle(pub Handle<Mesh>);

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        let mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Sphere::new(DOT_RADIUS));
        app.insert_resource(DotMeshHandle(mesh_handle)).add_systems(
            Update,
            handle_sketch_dot
                .run_if(in_state(SketchMode::Dot).and(input_just_pressed(MouseButton::Left))),
        );
    }
}

#[hot]
pub fn handle_sketch_dot(
    commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
) {
    spawn_dot(commands, dot_mesh, ui_materials, cursor.position);
}

#[hot]
pub fn spawn_dot(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    position: Vec3,
) {
    commands
        .spawn((
            Dot,
            Mesh3d(dot_mesh.0.clone()),
            MeshMaterial3d(ui_materials.dot.clone()),
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Transform::from_translation(position),
        ))
        .observe(update_material_on::<Pointer<Over>>(
            ui_materials.hover.clone(),
        ))
        .observe(update_material_on::<Pointer<Out>>(ui_materials.dot.clone()))
        .observe(update_material_on::<Pointer<Pressed>>(
            ui_materials.pressed.clone(),
        ))
        .observe(update_material_on::<Pointer<Released>>(
            ui_materials.hover.clone(),
        ));
}
