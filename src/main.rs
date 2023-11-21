use bevy::{input::mouse::MouseWheel, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Config::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, draw_cursor, zoom_in))
        .add_systems(FixedUpdate, sprite_movement)
        .run();
}

pub fn zoom_in(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
) {
    let mut y = 0.0;
    for event in mouse_wheel_events.read() {
        y += event.y;
    }
    for mut projection in query.iter_mut() {
        projection.scale -= y * time.delta_seconds();
    }
}

macro_rules! config {
    ($dbg: expr, $release: expr) => {{
        #[cfg(debug_assertions)]
        {
            $dbg
        }
        #[cfg(not(debug_assertions))]
        {
            $release
        }
    }};
}

#[cfg(not(debug_assertions))]
#[derive(Resource)]
struct Config {
    speed: f32,
    sprint_multiplier: f32,
}

#[cfg(debug_assertions)]
#[derive(Resource)]
struct Config;

#[cfg_attr(
    debug_assertions,
    const_tweaker::tweak(min = 100.0, max = 1000.0, step = 1.0)
)]
const PLAYER_SPEED: f32 = 150.0;

#[cfg_attr(
    debug_assertions,
    const_tweaker::tweak(min = 1.0, max = 100.0, step = 0.01)
)]
const SPRINT_MULTIPLIER: f32 = 1.5;

impl Config {
    #[cfg(not(debug_assertions))]
    pub fn new() -> Self {
        Self {
            speed: PLAYER_SPEED,
            sprint_multiplier: SPRINT_MULTIPLIER,
        }
    }
    #[cfg(debug_assertions)]
    pub fn new() -> Self {
        Self
    }
    #[inline(always)]
    pub fn speed(&self) -> f32 {
        config!(*PLAYER_SPEED, self.speed)
    }
    #[inline(always)]
    pub fn sprint_multiplier(&self) -> f32 {
        config!(*SPRINT_MULTIPLIER, self.sprint_multiplier)
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Deref)]
struct Acceleration(Vec3);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Acceleration(Default::default()),
        Player,
    ));
}

fn keyboard_input_system(
    config: Res<Config>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&Player, &mut Acceleration)>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    for (_, mut acc) in &mut sprite_position {
        (*acc).0.x = 0.0;
        (*acc).0.y = 0.0;
        if keyboard_input.pressed(KeyCode::A) {
            (*acc).0.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            (*acc).0.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            (*acc).0.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            (*acc).0.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Escape) {
            app_exit_events.send(bevy::app::AppExit);
        }

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            (*acc).0.x *= config.sprint_multiplier();
            (*acc).0.y *= config.sprint_multiplier();
        }
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    config: Res<Config>,
    mut sprite_position: Query<(&Acceleration, &mut Transform)>,
) {
    for (acc, mut transform) in &mut sprite_position {
        transform.translation += acc.0 * config.speed() * time.delta_seconds();
    }
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
