use bevy::{app::AppExit, prelude::*};

mod answer;
mod audio;
mod piece;
mod points;
mod timer;

use answer::{Answer, AnswerPlugin, InputText};
use audio::GameAudioPlugin;
use piece::{NewBoard, PiecePlugin, White};
use points::{Outcome, Points, PointsPlugin, PointsText};
use timer::{TimerPlugin, TimerText, TIMER_COLOR};

const BACKGROUND_COLOR: Color = Color::DARK_GRAY;

fn main() {
    // Forward Rust panics to browser console.
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    App::new()
        .insert_resource(WindowDescriptor {
            fit_canvas_to_parent: true,
            ..default()
        })
        .add_event::<Answer>()
        .add_event::<NewBoard>()
        .init_resource::<Ongoing>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(PiecePlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(AnswerPlugin)
        .add_plugin(PointsPlugin)
        .add_plugin(TimerPlugin)
        .add_startup_system(setup_system)
        .add_system(esc_to_exit)
        .add_system(check_answer_system)
        .add_system(restart_system)
        .run()
}

#[derive(Default)]
struct Ongoing(bool);

struct AudioHandles {
    click: Handle<AudioSource>,
    ding: Handle<AudioSource>,
    correct: Handle<AudioSource>,
    incorrect: Handle<AudioSource>,
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(AudioHandles {
        click: asset_server.load("sounds/click.ogg"),
        ding: asset_server.load("sounds/ding.ogg"),
        correct: asset_server.load("sounds/correct.ogg"),
        incorrect: asset_server.load("sounds/incorrect.ogg"),
    });
}

fn esc_to_exit(keys: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    #[cfg(not(target_arch = "wasm32"))]
    if keys.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn restart_system(
    keys: Res<Input<KeyCode>>,
    mut points: ResMut<Points>,
    mut event_writer: EventWriter<NewBoard>,
    mut ongoing: ResMut<Ongoing>,
    mut input_text: Query<&mut Text, (With<InputText>, Without<TimerText>)>,
    mut points_text: Query<&mut Visibility, (With<PointsText>, Without<TimerText>)>,
    mut timer_text: Query<(&mut Visibility, &mut Text), (With<TimerText>, Without<PointsText>)>,
    mut timer: ResMut<Timer>,
) {
    if keys.just_pressed(KeyCode::R) {
        // Request new board;
        event_writer.send(NewBoard);
        // Reset resources.
        *points = Points::default();
        *ongoing = Ongoing(true);
        // Clear input text.
        input_text.single_mut().sections[0].value.clear();
        // Make points text visible.
        points_text.single_mut().is_visible = true;

        // Reset timer.
        timer.reset();
        let (mut timer_text_vis, mut timer_text) = timer_text.single_mut();
        timer_text_vis.is_visible = true;
        timer_text.sections[0].style.color = TIMER_COLOR;
    }
}

fn check_answer_system(
    mut events: EventReader<Answer>,
    query: Query<&White>,
    mut outcome_event_writer: EventWriter<Outcome>,
    mut new_board_event_writer: EventWriter<NewBoard>,
) {
    for Answer(answer) in events.iter() {
        let white = query.iter().filter(|White(bool)| *bool).count();
        let black = query.iter().filter(|White(bool)| !*bool).count();
        let correct = white as i32 - black as i32;

        if *answer == correct {
            outcome_event_writer.send(Outcome::Correct);
        } else {
            outcome_event_writer.send(Outcome::Incorrect);
        }

        // Request new board after checking answer.
        new_board_event_writer.send(NewBoard);
    }
}
