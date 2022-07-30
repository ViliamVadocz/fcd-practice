use bevy::prelude::*;
use rand::Rng;

const BOARD_SIZE: u8 = 6;
const PADDING: f32 = 50.;
const PIECE_SIZE_RATIO: f32 = 0.8;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(position_system)
            .add_system(size_system)
            .add_system(generate_board_system);
    }
}

pub struct NewBoard;

#[derive(Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct White(pub bool);

#[derive(Bundle)]
struct PieceBundle {
    position: Position,
    white: White,

    #[bundle]
    sprite: SpriteBundle,
}

fn get_sizes(windows: Res<Windows>) -> (f32, f32) {
    windows
        .get_primary()
        .map(|window| {
            let height = window.height();
            let width = window.width();

            let board_size = width.min(height) - 2. * PADDING;
            let square_size = board_size / (BOARD_SIZE as f32);

            (board_size, square_size)
        })
        .unwrap_or_default()
}

fn position_system(mut query: Query<(&mut Transform, &Position)>, windows: Res<Windows>) {
    let (board_size, square_size) = get_sizes(windows);
    let (x0, y0) = (-board_size / 2., board_size / 2.);

    for (mut transform, Position { x, y }) in query.iter_mut() {
        transform.translation.x = x0 + (square_size * *x as f32) + (square_size / 2.);
        transform.translation.y = y0 - (square_size * *y as f32) - (square_size / 2.);
    }
}

fn size_system(mut query: Query<&mut Sprite>, windows: Res<Windows>) {
    let (_board_size, square_size) = get_sizes(windows);
    let piece_size = square_size * PIECE_SIZE_RATIO;

    for mut sprite in query.iter_mut() {
        sprite.custom_size = Some(Vec2::new(piece_size, piece_size))
    }
}

fn generate_board_system(
    mut commands: Commands,
    query: Query<Entity, With<Position>>,
    mut events: EventReader<NewBoard>,
) {
    for _event in events.iter() {
        // Delete all entities.
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        let mut rng = rand::thread_rng();

        // Spawn new board.
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                // Chance to have empty space.
                if rng.gen_ratio(1, 3) {
                    continue;
                }

                let is_white = rng.gen();
                commands.spawn_bundle(PieceBundle {
                    position: Position { x, y },
                    white: White(is_white),
                    sprite: SpriteBundle {
                        sprite: Sprite {
                            color: if is_white { Color::WHITE } else { Color::BLACK },
                            ..default()
                        },
                        ..default()
                    },
                });
            }
        }
    }
}
