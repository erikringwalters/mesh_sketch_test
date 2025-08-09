use bevy::prelude::*;

use super::colors::*;

#[derive(Resource, Default)]
pub struct UIMaterials {
    pub dot: Handle<StandardMaterial>,
    pub line: Handle<StandardMaterial>,
    pub hover: Handle<StandardMaterial>,
    // pub pressed: Handle<StandardMaterial>,
}
pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIMaterials::default())
            .add_systems(Startup, setup_ui_materials);
    }
}

pub fn setup_ui_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(UIMaterials {
        dot: materials.add(ui_material(CREAMSICLE_ORANGE)),
        line: materials.add(ui_material(LINE)),
        hover: materials.add(ui_material(HOVER)),
        // pressed: materials.add(ui_material(PRESSED)),
    });
}

pub fn ui_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        unlit: true,
        ..default()
    }
}
