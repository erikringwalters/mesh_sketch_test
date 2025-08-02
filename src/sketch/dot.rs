use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::*;

use crate::{
    assets::materials::UIMaterials,
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    size::DOT_RADIUS,
    sketch::{Selected, SketchMode, update_material_on},
};

#[derive(Component, Debug, Default)]
pub struct Dot;

#[derive(Resource, Debug)]
pub struct DotMeshHandle(pub Handle<Mesh>);

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        let dot_mesh_handle = app
            .world_mut()
            .resource_mut::<Assets<Mesh>>()
            .add(Sphere::new(DOT_RADIUS));
        app.insert_resource(DotMeshHandle(dot_mesh_handle))
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
pub fn spawn_temporary_dot(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Dot::default(),
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
) -> Entity {
    commands
        .entity(dot_entity)
        .insert(Mesh3d(dot_mesh.0.clone()))
        .insert(MeshMaterial3d(ui_materials.dot.clone()));

    setup_dot_observes(commands, ui_materials, dot_entity);
    return dot_entity;
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
        .observe(move_on_drag)
        .observe(handle_dot_select);
}

#[hot]
pub fn move_on_drag(
    drag: Trigger<Pointer<Drag>>,
    cursor: Res<Cursor>,
    state: ResMut<State<SketchMode>>,
    mut transforms: Query<&mut Transform>,
) {
    if state.get().ne(&SketchMode::None) {
        return;
    }
    let mut transform = transforms.get_mut(drag.target()).unwrap();
    transform.translation = cursor.position;
}

pub fn handle_dot_select(_click: Trigger<Pointer<Click>>, mut selected: ResMut<Selected>) {
    selected.dots.push(_click.target());
}
