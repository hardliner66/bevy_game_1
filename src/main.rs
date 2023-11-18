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

#[derive(Component)]
struct Direction {
    x: i32,
    y: i32,
    sprint: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction {
            x: 0,
            y: 0,
            sprint: false,
        },
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
        (*direction).sprint = false;
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
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            (*direction).sprint = true;
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
        let Direction { x, y, sprint } = *logo;
        let speed = if sprint {
            config.speed() * config.sprint_multiplier()
        } else {
            config.speed()
        };
        if y > 0 {
            transform.translation.y += speed * time.delta_seconds();
        }
        if y < 0 {
            transform.translation.y -= speed * time.delta_seconds();
        }

        if x > 0 {
            transform.translation.x += speed * time.delta_seconds();
        }
        if x < 0 {
            transform.translation.x -= speed * time.delta_seconds();
        }
    }
}
