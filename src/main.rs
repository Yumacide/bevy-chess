use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

const LIGHT_COLOR: Color = Color::rgb(1.0, 206.0 / 255.0, 158.0 / 255.0);
const DARK_COLOR: Color = Color::rgb(209.0 / 255.0, 137.0 / 255.0, 71.0 / 255.0);
const SQUARE_LENGTH: f32 = 80.0;

#[rustfmt::skip]
const PIECES: [u8; 64] = [
	1, 2, 3, 4, 5, 3, 2, 1,
	6, 6, 6, 6, 6, 6, 6, 6,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	7, 7, 7, 7, 7, 7, 7, 7,
	8, 9, 10, 11, 12, 10, 9, 8
];

#[derive(Component, Debug)]
enum Team {
    White,
    Black,
}

#[derive(Component, Debug)]
enum PieceType {
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Pawn,
}

#[derive(Component)]
struct Position(u8, u8);

#[derive(Bundle)]
struct Piece {
    team: Team,
    ty: PieceType,
    position: Position,
    sprite_bundle: SpriteBundle,
}

fn get_piece_image(team: Team, ty: PieceType) -> String {
    let team = match team {
        Team::Black => "black",
        Team::White => "white",
    };
    let ty = match ty {
        PieceType::Rook => "rook",
        PieceType::Knight => "knight",
        PieceType::Bishop => "bishop",
        PieceType::King => "king",
        PieceType::Queen => "queen",
        PieceType::Pawn => "pawn",
    };

    format!("pieces/{team}_{ty}.png")
}

fn spawn_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for (i, &n) in PIECES.iter().enumerate() {
        if n == 0 {
            continue;
        }

        let team = if n < 7 { Team::Black } else { Team::White };
        let ty = match n {
            1 | 8 => PieceType::Rook,
            2 | 9 => PieceType::Knight,
            3 | 10 => PieceType::Bishop,
            4 | 11 => PieceType::Queen,
            5 | 12 => PieceType::King,
            6 | 7 => PieceType::Pawn,
            _ => unreachable!(),
        };

        let x = (i % 8 + 1) as u8;
        let y = 9 - (i / 8 + 1) as u8;

        commands.spawn(SpriteBundle {
            texture: asset_server.load(get_piece_image(team, ty)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SQUARE_LENGTH, SQUARE_LENGTH)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                SQUARE_LENGTH * (x as f32) - SQUARE_LENGTH * 4.5,
                SQUARE_LENGTH * (y as f32) - SQUARE_LENGTH * 4.5,
                0.0,
            )),
            ..default()
        });
    }
}

fn spawn_board(mut commands: Commands) {
    for y in 1..=8 {
        for x in 1..=8 {
            let transform = Transform::from_translation(Vec3::new(
                SQUARE_LENGTH * (x as f32) - SQUARE_LENGTH * 4.5,
                SQUARE_LENGTH * (y as f32) - SQUARE_LENGTH * 4.5,
                0.0,
            ));

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: if x % 2 == 1 && y % 2 == 1 || x % 2 == 0 && y % 2 == 0 {
                        DARK_COLOR
                    } else {
                        LIGHT_COLOR
                    },
                    custom_size: Some(Vec2::new(SQUARE_LENGTH, SQUARE_LENGTH)),
                    ..default()
                },
                transform,
                ..default()
            });
        }
    }
}

fn move_piece(
    mut input: EventReader<MouseButtonInput>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };

    for event in input.iter() {
        if event.state == ButtonState::Pressed {}
    }
}

fn get_square(point: Vec2, window: &Window) -> Option<(u8, u8)> {
    let corner_x = (window.width() - SQUARE_LENGTH * 8.0) / 2.0;
    let corner_y = (window.height() + SQUARE_LENGTH * 8.0) / 2.0;
    let x = ((point.x - corner_x) / SQUARE_LENGTH).ceil() as u8;
    let y = ((corner_y - point.y) / SQUARE_LENGTH).ceil() as u8;

    if !(1..=8).contains(&x) || !(1..=8).contains(&y) {
        return None;
    }

    Some((x, y))
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_pieces)
        .add_system(move_piece)
        .run();
}
