use crate::actions::Actions;
use crate::constants;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(confine_player_movement.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_ball_blue_large.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = constants::SPRITE_SPEED;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = constants::SPRITE_SIZE / 2.;
        // bound the player to the window, when the camera is set to 0,0
        let x_min = -window.width() / 2. + half_player_size;
        let x_max = window.width() / 2. - half_player_size;
        let y_min = -window.height() / 2. + half_player_size;
        let y_max = window.height() / 2. - half_player_size;

        let mut translation = player_transform.translation;
        // bound the player to the window
        translation.x = translation.x.min(x_max).max(x_min);
        translation.y = translation.y.min(y_max).max(y_min);
        player_transform.translation = translation;
    }
}
