use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use rand;

pub const PLAYER_SIZE: f32 = 64.0;  // This is the player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player.after(spawn_camera))
        .add_systems(Startup, spawn_enemies.after(spawn_camera))
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement.after(player_movement))
        .run();
}


#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (
            Sprite {
                image: asset_server.load("/home/selene/bevy_tutorials/bevy_ball_game/assets/sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        )
    );
}


pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2d {
            ..default()
        }
    );
}


pub fn spawn_enemies(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {

        let random_x: f32 = window.width() * (rand::random::<f32>() - 0.5);  // Number between -window.width()/2 and window.width()/2
        let random_y: f32 = window.height() * (rand::random::<f32>() - 0.5); // Number between -window.height()/2 and window.height()/2
        
        commands.spawn(
            (
                Sprite {
                    image: asset_server.load("/home/selene/bevy_tutorials/bevy_ball_game/assets/sprites/ball_red_large.png"),
                    ..default()
                },
                Transform::from_xyz(random_x, random_y, 0.0),
                Enemy {},
            )
        );
    }
}




pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}


pub fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let half_player_size: f32 = PLAYER_SIZE / 2.0;  // 32.0

        let x_min = -window.width()/2.0 + half_player_size;
        let x_max = window.width()/2.0 - half_player_size;
        let y_min = -window.height()/2.0 + half_player_size;
        let y_max = window.height()/2.0 - half_player_size;

        let mut translation: Vec3 = player_transform.translation;
    
        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
