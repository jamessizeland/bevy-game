use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/scifi/spaceEngine_003.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(path = "audio/scifi/laserLarge_003.ogg")]
    pub shoot: Handle<AudioSource>,
    #[asset(path = "audio/scifi/explosionCrunch_003.ogg")]
    pub explosion: Handle<AudioSource>,
    #[asset(path = "audio/scifi/forceField_000.ogg")]
    pub force_field_0: Handle<AudioSource>,
    #[asset(path = "audio/scifi/forceField_001.ogg")]
    pub force_field_1: Handle<AudioSource>,
    #[asset(path = "audio/scifi/forceField_002.ogg")]
    pub force_field_2: Handle<AudioSource>,
    #[asset(path = "audio/scifi/forceField_003.ogg")]
    pub force_field_3: Handle<AudioSource>,
    #[asset(path = "audio/scifi/forceField_004.ogg")]
    pub force_field_4: Handle<AudioSource>,
    #[asset(path = "audio/interface/pluck_001.ogg")]
    pub pluck_001: Handle<AudioSource>,
    #[asset(path = "audio/interface/pluck_002.ogg")]
    pub pluck_002: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/ball_blue_large.png")]
    pub texture_ball_blue_large: Handle<Image>,
    #[asset(path = "textures/ball_red_large.png")]
    pub texture_ball_red_large: Handle<Image>,
    #[asset(path = "textures/star.png")]
    pub texture_star: Handle<Image>,
}
