use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::{
    assets::materials::UIMaterials,
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    size::{DOT_RADIUS, LINE_WIDTH},
    sketch::{SketchMode, update_material_on},
};

#[derive(Debug, Default)]
enum DotMode {
    #[default]
    Temporary,
    Permanent,
}

#[derive(Component, Debug, Default)]
pub struct Dot {
    mode: DotMode,
}

#[derive(Resource, Debug)]
pub struct DotMeshHandle(pub Handle<Mesh>);

#[derive(Resource, Debug)]
pub struct SketchDotMeshHandle(pub Handle<Mesh>);

// #[derive(Resource, Debug)]
// pub struct Connected

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        let dot_mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Sphere::new(DOT_RADIUS));
        let sketch_dot_mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Sphere::new(LINE_WIDTH));
        app.insert_resource(DotMeshHandle(dot_mesh_handle))
            .insert_resource(SketchDotMeshHandle(sketch_dot_mesh_handle))
            .add_systems(
                Update,
                spawn_dot
                    .run_if(in_state(SketchMode::Dot).and(input_just_pressed(MouseButton::Left))),
            );
    }
}

#[hot]
pub fn spawn_dot(
    mut commands: Commands,
    dot_mesh: Res<DotMeshHandle>,
    ui_materials: Res<UIMaterials>,
    cursor: Res<Cursor>,
) {
    let dot_entity = commands
        .spawn((
            Dot::default(),
            Mesh3d(dot_mesh.0.clone()),
            MeshMaterial3d(ui_materials.dot.clone()),
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Transform::from_translation(cursor.position),
        ))
        .id();
    setup_dot_observes(&mut commands, &ui_materials, dot_entity);
}

#[hot]
pub fn spawn_current_sketch_dot(
    commands: &mut Commands,
    sketch_dot_mesh: &mut Res<SketchDotMeshHandle>,
    ui_materials: &mut Res<UIMaterials>,
    position: Vec3,
) -> Entity {
    commands
        .spawn((
            Dot::default(),
            Mesh3d(sketch_dot_mesh.0.clone()),
            MeshMaterial3d(ui_materials.line.clone()),
            Reloadable {
                level: ReloadLevel::Hard,
            },
            Transform::from_translation(position),
        ))
        .id()
}

#[hot]
pub fn finalize_dot(
    commands: &mut Commands,
    dot_mesh: &Res<DotMeshHandle>,
    ui_materials: &Res<UIMaterials>,
    dot_entity: Entity,
    query: &mut Query<(&mut Dot, &mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    if let Ok((mut dot, mut mesh, mut material)) = query.get_mut(dot_entity) {
        dot.mode = DotMode::Permanent;
        *mesh = Mesh3d(dot_mesh.0.clone());
        *material = MeshMaterial3d(ui_materials.dot.clone())
    }
    setup_dot_observes(commands, ui_materials, dot_entity);
}

#[hot]
pub fn setup_dot_observes(
    commands: &mut Commands,
    ui_materials: &Res<UIMaterials>,
    dot_entity: Entity,
) {
    commands
        .entity(dot_entity)
        .observe(update_material_on::<Pointer<Over>>(
            ui_materials.hover.clone(),
        ))
        .observe(update_material_on::<Pointer<Out>>(ui_materials.dot.clone()))
        .observe(update_material_on::<Pointer<Pressed>>(
            ui_materials.pressed.clone(),
        ))
        .observe(update_material_on::<Pointer<Released>>(
            ui_materials.hover.clone(),
        ))
        .observe(move_on_drag);
}

#[hot]
pub fn move_on_drag(
    drag: Trigger<Pointer<Drag>>,
    cursor: Res<Cursor>,
    mut transforms: Query<&mut Transform>,
) {
    let mut transform = transforms.get_mut(drag.target()).unwrap();
    transform.translation = cursor.position;
}
