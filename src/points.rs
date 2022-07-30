use bevy::prelude::*;

use crate::restart_system;

const POINTS_COLOR: Color = Color::ANTIQUE_WHITE;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Outcome>()
            .init_resource::<Points>()
            .add_startup_system(setup_points_text_system)
            .add_system(track_points.before(restart_system))
            .add_system(points_text_system);
    }
}

pub enum Outcome {
    Correct,
    Incorrect,
}

#[derive(Default)]
pub struct Points {
    correct: u32,
    incorrect: u32,
}

#[derive(Component)]
pub struct PointsText;

fn setup_points_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle {
                text: Text::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/RobotoMono-Medium.ttf"),
                        font_size: 100.,
                        color: POINTS_COLOR,
                    },
                ),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            }
            .with_text_alignment(TextAlignment::TOP_RIGHT)
            .with_style(Style {
                align_self: AlignSelf::Center,
                position: UiRect {
                    right: Val::Px(50.),
                    top: Val::Px(10.),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            }),
        )
        .insert(PointsText);
}

fn track_points(mut points: ResMut<Points>, mut events: EventReader<Outcome>) {
    for outcome in events.iter() {
        if matches!(outcome, Outcome::Correct) {
            points.correct += 1;
        } else {
            points.incorrect += 1;
        }
    }
}

fn points_text_system(mut query: Query<&mut Text, With<PointsText>>, points: Res<Points>) {
    if points.is_changed() {
        query.single_mut().sections[0].value = points.correct.to_string();
    }
}
