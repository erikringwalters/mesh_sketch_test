use bevy::prelude::*;

use crate::assets::materials::ChangingMaterial;
use crate::cursor::Picking;

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Selected;

pub fn toggle_select_entity(
    mut commands: Commands,
    picking: Res<Picking>,
    query: Query<&Selected>,
) {
    let entity = picking.hovered;

    if entity == Entity::PLACEHOLDER {
        return;
    }

    if query.get(entity).is_ok() {
        // deselect already selected
        commands.entity(entity).remove::<Selected>();
    } else {
        // select not yet selected
        commands.entity(entity).insert(Selected);
    }
    commands.entity(entity).insert(ChangingMaterial);
}

pub fn deselect_other_entities(
    mut commands: Commands,
    picking: Res<Picking>,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Selected>>,
) {
    let is_chain_select = keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight)
        || keyboard.pressed(KeyCode::ControlLeft)
        || keyboard.pressed(KeyCode::ControlRight);
    if !is_chain_select {
        for entity in query.iter() {
            if entity == picking.hovered {
                continue;
            }
            commands.entity(entity).remove::<Selected>();
            commands.entity(entity).insert(ChangingMaterial);
        }
    }
}
