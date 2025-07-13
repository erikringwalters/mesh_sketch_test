use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Vec3,
}
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor::default())
            .add_systems(Update, update_cursor); //, draw_cursor));
    }
}

#[hot]
fn update_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursor: ResMut<Cursor>,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the floor plane.
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) else {
        return;
    };
    cursor.position = ray.get_point(distance);
}

#[hot]
fn draw_cursor(mut gizmos: Gizmos, cursor: Res<Cursor>) {
    gizmos.circle(
        Isometry3d::new(
            cursor.position + Dir3::Y * 0.,
            Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
        ),
        0.05,
        Color::WHITE,
    );
}
