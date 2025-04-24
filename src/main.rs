use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand;

pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size
pub const ENEMY_SIZE: f32 = 64.0; // This is the player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: usize = 20;
pub const ENEMY_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player.after(spawn_camera))
        .add_systems(Startup, spawn_enemies.after(spawn_camera))
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement.after(player_movement))
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction.after(enemy_movement))
        .add_systems(Update, confine_enemy_movement.after(update_enemy_direction))
        .add_systems(Update, enemy_hit_player.after(enemy_movement))
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load(
                "/home/selene/bevy_tutorials/bevy_ball_game/assets/sprites/ball_blue_large.png",
            ),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}

pub fn spawn_enemies(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = window.width() * (rand::random::<f32>() - 0.5); // Number between -window.width()/2 and window.width()/2
        let random_y: f32 = window.height() * (rand::random::<f32>() - 0.5); // Number between -window.height()/2 and window.height()/2

        commands.spawn((
            Sprite {
                image: asset_server.load(
                    "/home/selene/bevy_tutorials/bevy_ball_game/assets/sprites/ball_red_large.png",
                ),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(rand::random::<f32>(), rand::random::<f32>()).normalize(), // Normalize the direction to make it a unit vector
            },
        ));
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
        let half_player_size: f32 = PLAYER_SIZE / 2.0; // 32.0

        let x_min = -window.width() / 2.0 + half_player_size;
        let x_max = window.width() / 2.0 - half_player_size;
        let y_min = -window.height() / 2.0 + half_player_size;
        let y_max = window.height() / 2.0 - half_player_size;

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

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0; // 32.0

    let x_min: f32 = -window.width() / 2.0 + half_enemy_size;
    let x_max: f32 = window.width() / 2.0 - half_enemy_size;
    let y_min: f32 = -window.height() / 2.0 + half_enemy_size;
    let y_max: f32 = window.height() / 2.0 - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;

        let translation: Vec3 = transform.translation;

        // Check if the enemy is out of bounds
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server
                .load("/home/selene/bevy_tutorials/bevy_ball_game/assets/audio/pluck_001.ogg");
            let sound_effect_2 = asset_server
                .load("/home/selene/bevy_tutorials/bevy_ball_game/assets/audio/pluck_002.ogg");

            // Randomly play one of the two sound effects.

            let sound_effect = if rand::random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn((AudioPlayer::new(sound_effect), PlaybackSettings::DESPAWN));
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_transform_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for mut enemy_transform in enemy_transform_query.iter_mut() {
        let half_enemy_size: f32 = PLAYER_SIZE / 2.0; // 32.0

        let x_min = -window.width() / 2.0 + half_enemy_size;
        let x_max = window.width() / 2.0 - half_enemy_size;
        let y_min = -window.height() / 2.0 + half_enemy_size;
        let y_max = window.height() / 2.0 - half_enemy_size;

        let mut translation: Vec3 = enemy_transform.translation;

        // Bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        enemy_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance: f32 = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let enemy_radius: f32 = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("/home/selene/bevy_tutorials/bevy_ball_game/assets/audio/explosionCrunch_000.ogg");

                commands.spawn((AudioPlayer::new(sound_effect), PlaybackSettings::DESPAWN));

                commands.entity(player_entity).despawn();
            }
        }
    }
}
