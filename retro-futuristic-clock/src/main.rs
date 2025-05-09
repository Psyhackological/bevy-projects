use bevy::prelude::*;
use chrono::Timelike;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clock_face)
        .run();
}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    commands.spawn(Camera2d);

    let (default_config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    default_config.line.width = 20.0;
}

fn clock_face(mut gizmos: Gizmos) {
    let now = chrono::Local::now();

    let second = now.second() as f32;
    let minute = now.minute() as f32;
    let hour = now.hour() as f32;

    let second_angle = -(360.0 / 60.0) * second;
    let minute_angle = -(360.0 / 60.0) * minute;
    let hour_angle: f32 = -(360.0 / 12.0) * (hour % 12.);

    // Seconds
    gizmos
        .arc_2d(
            Isometry2d::new(
                Vec2::ZERO,
                Rot2::radians(second_angle.to_radians() / 2.0 + second_angle.to_radians() * 0.5),
            ),
            second_angle.to_radians(),
            100.,
            Color::srgb(1., 0.894, 0.769), // Bisque
        )
        .resolution(360 * 3);

    // Minutes
    gizmos
        .arc_2d(
            Isometry2d::new(
                Vec2::ZERO,
                Rot2::radians(minute_angle.to_radians() / 2.0 + minute_angle.to_radians() * 0.5),
            ),
            minute_angle.to_radians(),
            120.,
            Color::srgb(0., 0.5, 0.5), // Teal
        )
        .resolution(360 * 3);

    // Hour
    gizmos
        .arc_2d(
            Isometry2d::new(
                Vec2::ZERO,
                Rot2::radians(hour_angle.to_radians() / 2.0 + hour_angle.to_radians() * 0.5),
            ),
            hour_angle.to_radians(),
            140.,
            Color::srgb(1., 0.647, 0.), // Orange
        )
        .resolution(360 * 3);
}
