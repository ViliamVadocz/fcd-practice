use bevy::prelude::*;

use crate::Ongoing;

pub const TIMER_COLOR: Color = Color::ANTIQUE_WHITE;
const TIMER_SECONDS: f32 = 5.;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Timer::from_seconds(TIMER_SECONDS, false))
            .add_startup_system(setup_timer_text_system)
            .add_system(update_timer_system)
            .add_system(time_up_system)
            .add_system(update_timer_text_system);
    }
}

#[derive(Component)]
pub struct TimerText;

fn setup_timer_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle {
                text: Text::from_section(
                    "00.00",
                    TextStyle {
                        font: asset_server.load("fonts/RobotoMono-Medium.ttf"),
                        font_size: 100.,
                        color: TIMER_COLOR,
                    },
                ),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            }
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::Center,
                position: UiRect {
                    left: Val::Px(50.),
                    top: Val::Px(10.),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            }),
        )
        .insert(TimerText);
}

fn update_timer_system(mut timer: ResMut<Timer>, time: Res<Time>, ongoing: Res<Ongoing>) {
    if ongoing.0 {
        timer.tick(time.delta());
    }
}

pub(crate) fn time_up_system(
    timer: Res<Timer>,
    mut ongoing: ResMut<Ongoing>,
    mut query: Query<&mut Text, With<TimerText>>,
) {
    if timer.just_finished() && ongoing.0 {
        *ongoing = Ongoing(false);
        // Chance timer color to RED.
        query.single_mut().sections[0].style.color = Color::RED;
    }
}

fn update_timer_text_system(mut query: Query<&mut Text, With<TimerText>>, timer: Res<Timer>) {
    let text = &mut query.single_mut().sections[0].value;

    let time_left = timer.duration() - timer.elapsed();

    let seconds = time_left.as_secs();
    let hundredths = (time_left.as_millis() - (seconds as u128 * 1000)) / 10;

    *text = format!("{seconds:0>2}.{hundredths:0>2}");
}
