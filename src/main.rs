mod assets;
mod cursor;
mod reload;
mod sketch;

use assets::materials::MaterialsPlugin;
// use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
// use bevy::input::common_conditions::input_just_pressed;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_simple_subsecond_system::*;
use cursor::CursorPlugin;
use reload::{ReloadPlugin, Reloadable};
use sketch::sketch::SketchPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            filter: "bevy_dev_tools=trace".into(), // Show picking logs trace level and up
            ..default()
        }))
        .add_plugins(MeshPickingPlugin)
        // .add_plugins(DebugPickingPlugin)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(CursorPlugin)
        .add_plugins(ReloadPlugin)
        .add_plugins(SketchPlugin)
        .add_plugins(MaterialsPlugin)
        // .insert_resource(DebugPickingMode::Normal)
        .add_systems(Startup, setup)
        // .add_systems(
        //     PreUpdate,
        //     (|mut mode: ResMut<DebugPickingMode>| {
        //         *mode = match *mode {
        //             DebugPickingMode::Disabled => DebugPickingMode::Normal,
        //             DebugPickingMode::Normal => DebugPickingMode::Noisy,
        //             DebugPickingMode::Noisy => DebugPickingMode::Disabled,
        //         }
        //     })
        //     .distributive_run_if(input_just_pressed(KeyCode::F3)),
        // )
        .run();
}

#[hot]
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Dir3::Y),
        Reloadable::default(),
    ));

    commands.spawn((DirectionalLight::default(), Reloadable::default()));
}
