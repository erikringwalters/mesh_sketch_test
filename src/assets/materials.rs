use bevy::prelude::*;

use crate::cursor::Picking;
use crate::schedule::ScheduleSet;
use crate::sketching::dot::Dot;
use crate::sketching::line::Line;
use crate::sketching::selection::Selected;

use super::colors::*;

type ChangingButNotSelected<T> = (With<T>, With<ChangingMaterial>, Without<Selected>);
type ChangingAndSelected = (With<ChangingMaterial>, With<Selected>);

#[derive(Resource, Default)]
pub struct UIMaterials {
    pub dot: Handle<StandardMaterial>,
    pub line: Handle<StandardMaterial>,
    pub hover: Handle<StandardMaterial>,
    pub selected: Handle<StandardMaterial>,
    pub selected_and_hovered: Handle<StandardMaterial>,
}

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct ChangingMaterial;

pub trait UIMaterialProvider {
    fn get_material(ui_materials: &UIMaterials) -> Handle<StandardMaterial>;
}

pub fn get_ui_material<T: Component + UIMaterialProvider>(
    ui_materials: &Res<UIMaterials>,
) -> Handle<StandardMaterial> {
    T::get_material(ui_materials)
}

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIMaterials::default())
            .add_systems(PreStartup, setup_ui_materials)
            .add_systems(
                Update,
                (
                    update_to_selected_material,
                    update_to_hover_material::<Dot>,
                    update_to_hover_material::<Line>,
                    update_to_default_material::<Dot>,
                    update_to_default_material::<Line>,
                )
                    .chain()
                    .in_set(ScheduleSet::EntityUpdates),
            );
    }
}

pub fn setup_ui_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(UIMaterials {
        dot: materials.add(ui_material(color_from_hex(CREAMSICLE_ORANGE))),
        line: materials.add(ui_material(color_from_hex(LINE))),
        hover: materials.add(ui_material(color_from_hex(HOVER))),
        selected: materials.add(ui_material(color_from_hex(LEAF_GREEN))),
        selected_and_hovered: materials.add(ui_material(color_from_hex(DARK_SEAFOAM))),
    });
}

pub fn ui_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        unlit: true,
        ..default()
    }
}

pub fn update_to_default_material<T: Component + UIMaterialProvider>(
    mut commands: Commands,
    ui_materials: Res<UIMaterials>,
    mut material_query: Query<
        (Entity, &mut MeshMaterial3d<StandardMaterial>),
        ChangingButNotSelected<T>,
    >,
) {
    for (entity, mut material) in material_query.iter_mut() {
        let mat = get_ui_material::<T>(&ui_materials);
        material.0 = mat;
        commands.entity(entity).remove::<ChangingMaterial>();
    }
}

pub fn update_to_hover_material<T: Component>(
    mut commands: Commands,
    picking: ResMut<Picking>,
    ui_materials: Res<UIMaterials>,
    mut material_query: Query<&mut MeshMaterial3d<StandardMaterial>, ChangingButNotSelected<T>>,
) {
    if let Ok(mut material) = material_query.get_mut(picking.hovered) {
        material.0 = ui_materials.hover.clone();
        commands
            .entity(picking.hovered)
            .remove::<ChangingMaterial>();
    }
}

pub fn update_to_selected_material(
    mut commands: Commands,
    picking: ResMut<Picking>,
    ui_materials: Res<UIMaterials>,
    mut query: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>), ChangingAndSelected>,
) {
    for (entity, mut material) in query.iter_mut() {
        if entity == picking.hovered {
            material.0 = ui_materials.selected_and_hovered.clone();
        } else {
            material.0 = ui_materials.selected.clone();
        }
        commands.entity(entity).remove::<ChangingMaterial>();
    }
}
