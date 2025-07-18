mod color;
mod cursor;
mod reload;
mod sketch;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_simple_subsecond_system::*;
use cursor::CursorPlugin;
use reload::{ReloadPlugin, Reloadable};
use sketch::sketch::SketchPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(CursorPlugin)
        .add_plugins(ReloadPlugin)
        .add_plugins(SketchPlugin)
        .add_systems(Startup, setup)
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
