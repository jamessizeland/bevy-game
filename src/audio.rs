use crate::actions::{set_movement_actions, set_player_shoot, Actions};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(
                control_flying_sound
                    .after(set_movement_actions)
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(
                control_shoot_sound
                    .after(set_player_shoot)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct AudioInstances {
    flying: Handle<AudioInstance>,
    shooting: Handle<AudioInstance>,
    bouncing: Handle<AudioInstance>,
}

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    fn gen_instance(audio: &Res<Audio>, handle: &Handle<AudioSource>) -> Handle<AudioInstance> {
        audio
            .play(handle.clone())
            .looped()
            .with_volume(0.3)
            .handle()
    }
    commands.insert_resource(AudioInstances {
        flying: gen_instance(&audio, &audio_assets.flying),
        shooting: gen_instance(&audio, &audio_assets.shoot),
        bouncing: gen_instance(&audio, &audio_assets.force_field_3),
    });
}

fn control_flying_sound(
    actions: Res<Actions>,
    audio: Res<AudioInstances>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.flying) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}

fn control_shoot_sound(actions: Res<Actions>, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    if actions.player_shoot {
        audio.play(audio_assets.shoot.clone());
    }
}
