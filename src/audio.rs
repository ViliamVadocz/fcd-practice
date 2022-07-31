use bevy::prelude::*;

use crate::{timer::time_up_system, AudioHandles, Ongoing, Outcome};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(click)
            .add_system(correct_sound)
            .add_system(time_is_up.before(time_up_system));
    }
}

fn click(audio: Res<Audio>, audio_handles: Res<AudioHandles>, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        let variation: f32 = rand::random::<f32>() / 2.;
        audio.play_with_settings(
            audio_handles.click.clone_weak(),
            PlaybackSettings {
                speed: 0.8 + variation,
                volume: 0.5,
                ..default()
            },
        );
    }
}

fn correct_sound(
    mut events: EventReader<Outcome>,
    audio_handles: Res<AudioHandles>,
    audio: Res<Audio>,
) {
    for outcome in events.iter() {
        match outcome {
            Outcome::Correct => audio.play(audio_handles.correct.clone_weak()),
            Outcome::Incorrect => audio.play(audio_handles.incorrect.clone_weak()),
        };
    }
}

fn time_is_up(
    timer: Res<Timer>,
    ongoing: Res<Ongoing>,
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
) {
    if timer.just_finished() && ongoing.0 {
        // Play ding!
        audio.play_with_settings(
            audio_handles.ding.clone_weak(),
            PlaybackSettings {
                volume: 0.5, // originally very loud and very scary
                ..default()
            },
        );
    }
}
