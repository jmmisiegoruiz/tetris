use crate::resources::Assets;
use crate::constants::{BOARD_WIDTH, BOARD_HEIGHT};
use ggez::{Context, GameResult, graphics};
use ggez::graphics::Text;
use std::convert::TryFrom;
use crate::world::{Tetrimino, TetriminoType, ScoreBoard, Board};
use crate::types::{ScreenPoint2, WorldPoint2};

pub fn draw_tetrimino(
    assets: &mut Assets,
    ctx: &mut Context,
    tetrimino: &Tetrimino,
    board_dimensions: (f32, f32),
    offset: Option<(f32, f32)>,
) -> GameResult {
    let (board_w, board_h) = board_dimensions;
    let mut pos = world_to_screen_coords(board_w, board_h, &tetrimino.pos);
    if let Some((x_offset, y_offset)) = offset {
        pos = ScreenPoint2::from([pos.x + x_offset, pos.y + y_offset]);
    };
    let image = assets.block_image(TetriminoType::to_code(&tetrimino.kind)).unwrap();

    for vector in tetrimino.vectors.iter() {
        let mut vector_pos =
            world_to_screen_coords(
                board_w,
                board_h,
                &WorldPoint2::from([vector.x + tetrimino.pos.x, vector.y + tetrimino.pos.y]));
        if let Some((x_offset, y_offset)) = offset {
            vector_pos = ScreenPoint2::from([vector_pos.x + x_offset, vector_pos.y + y_offset]);
        };
        let draw_params = graphics::DrawParam::new()
            .scale([0.5, 0.5])
            .dest(vector_pos);
        graphics::draw(ctx, image, draw_params)?
    }

    let draw_params = graphics::DrawParam::new()
        .scale([0.5, 0.5])
        .dest(pos);
    graphics::draw(ctx, image, draw_params)
}

pub fn draw_board(
    assets: &mut Assets,
    ctx: &mut Context,
    board: &Board,
    board_dimensions: (f32, f32),
) -> GameResult {
    let (board_width, board_height) = board_dimensions;
    for (r, row) in board.data.row_iter().enumerate() {
        for (c, _element) in row.column_iter().enumerate() {
            let point =
                world_to_screen_coords(
                    board_width,
                    board_height,
                    &WorldPoint2::from([
                        i8::try_from(c).expect("Failed to convert X coordinate"),
                        i8::try_from(r).expect("Failed to convert Y coordinate"),
                    ]));
            let image = assets.block_image(*_element.get((0, 0)).unwrap());
            if let Some(image) = image {
                let draw_params = graphics::DrawParam::new()
                    .scale([0.5, 0.5])
                    .dest(point);
                graphics::draw(ctx, image, draw_params)?
            }
        }
    }
    GameResult::Ok(())
}

pub fn draw_score_board(
    ctx: &mut Context,
    score_board: &ScoreBoard,
) -> GameResult {
    let lines = Text::new(format!("LINES: {}", score_board.lines));
    let score = Text::new(format!("SCORE: {}", score_board.score));
    let level = Text::new(format!("LEVEL: {}", score_board.level));
    let next_piece = Text::new("NEXT_PIECE");

    graphics::draw(
        ctx,
        &lines,
        (ScreenPoint2::new(BOARD_WIDTH * 2.0 + BOARD_WIDTH / 8.0, 2.0 * BOARD_HEIGHT / 8.0), graphics::WHITE),
    )?;

    graphics::draw(
        ctx,
        &score,
        (ScreenPoint2::new(BOARD_WIDTH * 2.0 + BOARD_WIDTH / 8.0, 3.0 * BOARD_HEIGHT / 8.0), graphics::WHITE),
    )?;

    graphics::draw(
        ctx,
        &level,
        (ScreenPoint2::new(BOARD_WIDTH * 2.0 + BOARD_WIDTH / 8.0, 4.0 * BOARD_HEIGHT / 8.0), graphics::WHITE),
    )?;

    graphics::draw(
        ctx,
        &next_piece,
        (ScreenPoint2::new(BOARD_WIDTH * 2.0 + BOARD_WIDTH / 8.0, 5.5 * BOARD_HEIGHT / 8.0), graphics::WHITE),
    )
}

fn world_to_screen_coords(board_width: f32, board_height: f32, point: &WorldPoint2) -> ScreenPoint2 {
    let x = (point.x as f32) * (board_width / 12.0) + BOARD_WIDTH;
    let y = (point.y as f32) * (board_height / 21.0) + BOARD_HEIGHT / 4.0;
    ScreenPoint2::new(x, y)
}
