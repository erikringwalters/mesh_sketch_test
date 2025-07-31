use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_simple_subsecond_system::hot;

use crate::{
    setup,
    sketch::sketch::{self, CurrentSketch},
};

#[derive(Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ReloadLevel {
    #[default]
    Soft,
    Hard,
}

#[derive(Component, Default)]
pub struct Reloadable {
    pub level: ReloadLevel,
}

pub struct ReloadPlugin;

impl Plugin for ReloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_reload.run_if(input_just_pressed(KeyCode::Delete)),
                handle_setup.run_if(input_just_pressed(KeyCode::Delete)),
            )
                .chain(),
        );
    }
}

#[hot]
fn handle_reload(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, &Reloadable)>,
    current_sketch: ResMut<CurrentSketch>,
) {
    if input.pressed(KeyCode::ControlLeft) {
        let reload_level = if input.pressed(KeyCode::ShiftLeft) {
            ReloadLevel::Hard
        } else {
            ReloadLevel::Soft
        };
        for (entity, reloadable) in query.iter() {
            if reloadable.level <= reload_level {
                commands.entity(entity).despawn();
            }
        }
        sketch::reset_current_sketch(commands, current_sketch);
        let message = if reload_level == ReloadLevel::Soft {
            "Soft reloaded."
        } else {
            "Hard reloaded."
        };
        println!("{:?}", message);
    }
}

fn handle_setup(input: Res<ButtonInput<KeyCode>>, commands: Commands) {
    if input.pressed(KeyCode::ControlLeft) {
        setup(commands);
    }
}
