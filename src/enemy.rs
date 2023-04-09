use crate::constants;
use crate::loading::{AudioAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};
use rand::random;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

/// This plugin handles enemy related stuff like movement
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_enemy.in_set(OnUpdate(GameState::Playing)))
            .add_system(confine_enemy.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_enemy_direction.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    textures: Res<TextureAssets>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..constants::NO_ENEMIES {
        let x = rand::random::<f32>() * window.width() - window.width() / 2.;
        let y = rand::random::<f32>() * window.height() - window.height() / 2.;
        commands
            .spawn(SpriteBundle {
                texture: textures.texture_ball_red_large.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                ..default()
            })
            .insert(Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            });
    }
}

pub fn move_enemy(time: Res<Time>, mut query: Query<(&Enemy, &mut Transform)>) {
    for (enemy, mut transform) in query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * constants::ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemy(
    mut query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = (constants::ENEMY_SIZE / 2.) - 1.;
    let x_min = -window.width() / 2. + half_enemy_size;
    let x_max = window.width() / 2. - half_enemy_size;
    let y_min = -window.height() / 2. + half_enemy_size;
    let y_max = window.height() / 2. - half_enemy_size;
    for mut transform in query.iter_mut() {
        let mut translation = transform.translation;
        translation.x = translation.x.min(x_max).max(x_min);
        translation.y = translation.y.min(y_max).max(y_min);
        transform.translation = translation;
    }
}

pub fn update_enemy_direction(
    mut query: Query<(&mut Enemy, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = constants::ENEMY_SIZE / 2.;
    let x_min = -window.width() / 2. + half_enemy_size;
    let x_max = window.width() / 2. - half_enemy_size;
    let y_min = -window.height() / 2. + half_enemy_size;
    let y_max = window.height() / 2. - half_enemy_size;
    let sound_effects = vec![
        audio_assets.force_field_0.clone(),
        audio_assets.force_field_1.clone(),
        audio_assets.force_field_2.clone(),
        audio_assets.force_field_3.clone(),
        audio_assets.force_field_4.clone(),
    ];
    for (mut enemy, transform) in query.iter_mut() {
        let mut direction_changed = false;
        let x = transform.translation.x;
        let y = transform.translation.y;
        if x < x_min || x > x_max {
            enemy.direction.x *= -1.;
            direction_changed = true;
        } else if y < y_min || y > y_max {
            enemy.direction.y *= -1.;
            direction_changed = true;
        }
        if direction_changed {
            println!("bounced");
            audio.play(sound_effects[random::<usize>() % sound_effects.len()].clone());
        }
    }
}
