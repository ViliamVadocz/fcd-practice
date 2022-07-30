use bevy::prelude::*;

use crate::Outcome;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(click).add_system(correct_sound);
    }
}

fn click(asset_server: Res<AssetServer>, audio: Res<Audio>, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        let variation: f32 = rand::random::<f32>() / 2.;
        audio.play_with_settings(
            asset_server.load("sounds/click.ogg"),
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
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for outcome in events.iter() {
        match outcome {
            Outcome::Correct => audio.play(asset_server.load("sounds/correct.ogg")),
            Outcome::Incorrect => audio.play(asset_server.load("sounds/incorrect.ogg")),
        };
    }
}
