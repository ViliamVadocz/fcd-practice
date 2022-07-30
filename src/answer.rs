use std::ops::Not;

use bevy::prelude::*;

use crate::Ongoing;

const INPUT_TEXT_COLOR: Color = Color::ORANGE_RED;

pub struct AnswerPlugin;

impl Plugin for AnswerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_input_text_system)
            .add_system(fcd_input_system);
    }
}

#[derive(Component)]
pub struct InputText;

fn spawn_input_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "R to start",
                TextStyle {
                    font: asset_server.load("fonts/RobotoMono-Medium.ttf"),
                    font_size: 200.,
                    color: INPUT_TEXT_COLOR,
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            }),
        )
        .insert(InputText);
}

pub struct Answer(pub i32);

fn fcd_input_system(
    keys: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<Answer>,
    mut query: Query<(&mut Text, &mut Visibility), With<InputText>>,
    mut number: Local<u16>,
    mut minus: Local<bool>,
    ongoing: Res<Ongoing>,
) {
    if !ongoing.0 {
        return;
    }

    let (mut text, mut visibility) = query.single_mut();
    let text = &mut text.sections[0].value;
    let mut changed = false;

    for key in keys.get_just_pressed() {
        if let Some(digit) = match key {
            KeyCode::Key0 | KeyCode::Numpad0 => Some(0),
            KeyCode::Key1 | KeyCode::Numpad1 => Some(1),
            KeyCode::Key2 | KeyCode::Numpad2 => Some(2),
            KeyCode::Key3 | KeyCode::Numpad3 => Some(3),
            KeyCode::Key4 | KeyCode::Numpad4 => Some(4),
            KeyCode::Key5 | KeyCode::Numpad5 => Some(5),
            KeyCode::Key6 | KeyCode::Numpad6 => Some(6),
            KeyCode::Key7 | KeyCode::Numpad7 => Some(7),
            KeyCode::Key8 | KeyCode::Numpad8 => Some(8),
            KeyCode::Key9 | KeyCode::Numpad9 => Some(9),
            _ => None,
        } {
            // Type digit.
            if let Some(n) = number.checked_mul(10).and_then(|n| n.checked_add(digit)) {
                *number = n;
                changed = true;
            }
        } else if matches!(key, KeyCode::Minus) {
            // Negate input.
            *minus = minus.not();
            changed = true;
        } else if matches!(key, KeyCode::Back) {
            // Remove digit.
            if *number == 0 {
                *minus = false;
            }
            *number /= 10;
            changed = true;
        } else if matches!(key, KeyCode::Up | KeyCode::Right) {
            // Increment number.
            *number = match (*minus, number.checked_sub(1), number.checked_add(1)) {
                (true, Some(n), _) => n,
                (true, None, _) => {
                    *minus = false;
                    1
                }
                (false, _, Some(n)) => n,
                _ => *number,
            };
            changed = true;
        } else if matches!(key, KeyCode::Down | KeyCode::Left) {
            // Decrement number.
            *number = match (*minus, number.checked_sub(1), number.checked_add(1)) {
                (true, _, Some(n)) => n,
                (false, Some(n), _) => n,
                (false, None, _) => {
                    *minus = true;
                    1
                }
                _ => *number,
            };
            changed = true;
        } else if matches!(key, KeyCode::Return | KeyCode::Space) {
            // Send answer.
            if !text.is_empty() {
                event_writer.send(Answer(*number as i32 * if *minus { -1 } else { 1 }));
            }
        }

        // Reset after send or restart.
        if matches!(key, KeyCode::Return | KeyCode::Space | KeyCode::R) {
            *number = 0;
            *minus = false;
            text.clear();
            visibility.is_visible = false;
        }
    }

    if changed {
        *text = format!("{}{}", if *minus { "-" } else { "" }, *number);
        visibility.is_visible = true;
    }
}
