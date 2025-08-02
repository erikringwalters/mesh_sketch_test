use bevy::prelude::*;

use super::colors::{DOT_COLOR, HOVER_COLOR, PRESSED_COLOR};

#[derive(Resource, Default)]
pub struct UIMaterials {
    pub dot: Handle<StandardMaterial>,
    // pub line: Handle<StandardMaterial>,
    pub hover: Handle<StandardMaterial>,
    pub pressed: Handle<StandardMaterial>,
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
        dot: materials.add(ui_material(DOT_COLOR)),
        // line: materials.add(ui_material(LINE_COLOR)),
        hover: materials.add(ui_material(HOVER_COLOR)),
        pressed: materials.add(ui_material(PRESSED_COLOR)),
    });
}

pub fn ui_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        unlit: true,
        ..default()
    }
}
