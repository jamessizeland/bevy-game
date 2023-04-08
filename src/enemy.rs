use crate::constants;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

/// This plugin handles enemy related stuff like movement
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(GameState::Playing)));
        // .add_system(move_enemy.in_set(OnUpdate(GameState::Playing)))
        // .add_system(confine_enemy_movement.in_set(OnUpdate(GameState::Playing)));
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
            .insert(Enemy);
    }
}
