use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Config::new())
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Resource)]
struct Config {
    #[cfg(not(debug_assertions))]
    speed: f32,
}

#[cfg_attr(
    debug_assertions,
    const_tweaker::tweak(min = 100.0, max = 1000.0, step = 1.0)
)]
const PLAYER_SPEED: f32 = 150.0;

impl Config {
    pub fn new() -> Self {
        Self {
            #[cfg(not(debug_assertions))]
            speed: PLAYER_SPEED,
        }
    }
    #[inline(always)]
    pub fn speed(&self) -> f32 {
        #[cfg(debug_assertions)]
        {
            *PLAYER_SPEED
        }
        #[cfg(not(debug_assertions))]
        {
            self.speed
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Direction {
    x: i32,
    y: i32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction { x: 0, y: 0 },
        Player,
    ));
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&Player, &mut Direction)>,
) {
    for (_, mut direction) in &mut sprite_position {
        (*direction).x = 0;
        (*direction).y = 0;
        if keyboard_input.pressed(KeyCode::A) {
            (*direction).x = -1;
        }
        if keyboard_input.pressed(KeyCode::D) {
            (*direction).x = 1;
        }
        if keyboard_input.pressed(KeyCode::W) {
            (*direction).y = 1;
        }
        if keyboard_input.pressed(KeyCode::S) {
            (*direction).y = -1;
        }
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    config: Res<Config>,
    mut sprite_position: Query<(&Direction, &mut Transform)>,
) {
    for (logo, mut transform) in &mut sprite_position {
        let Direction { x, y } = *logo;
        if y > 0 {
            transform.translation.y += config.speed() * time.delta_seconds();
        }
        if y < 0 {
            transform.translation.y -= config.speed() * time.delta_seconds();
        }

        if x > 0 {
            transform.translation.x += config.speed() * time.delta_seconds();
        }
        if x < 0 {
            transform.translation.x -= config.speed() * time.delta_seconds();
        }
    }
}
