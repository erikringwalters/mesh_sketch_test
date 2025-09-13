mod assets;
mod constraints;
mod cursor;
mod reload;
mod schedule;
mod sketching;

use self::schedule::SchedulePlugin;
use assets::materials::MaterialsPlugin;
use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use cursor::CursorPlugin;
use reload::{ReloadPlugin, Reloadable};
use sketching::sketch::SketchPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mesh Sketch Test".into(),
                present_mode: PresentMode::AutoVsync,

                ..default()
            }),
            ..default()
        }))
        .add_plugins(SchedulePlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(ReloadPlugin)
        .add_plugins(SketchPlugin)
        .add_plugins(MaterialsPlugin)
        .add_systems(Startup, setup)
        .run();
}

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
