use ggez::{graphics, Context, GameResult, audio};
use ggez::graphics::{Image, Font};
use crate::world::{TetriminoType, BoardType};

pub struct Assets {
    i_block_image: graphics::Image,
    j_block_image: graphics::Image,
    l_block_image: graphics::Image,
    o_block_image: graphics::Image,
    s_block_image: graphics::Image,
    t_block_image: graphics::Image,
    pub z_block_image: graphics::Image,
    pub b_block_image: graphics::Image,
    pub font: Font,
    pub theme: audio::Source
}

impl Assets {
    pub(crate) fn new(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            i_block_image: graphics::Image::new(ctx, "/i_block.png")?,
            j_block_image: graphics::Image::new(ctx, "/j_block.png")?,
            l_block_image: graphics::Image::new(ctx, "/l_block.png")?,
            o_block_image: graphics::Image::new(ctx, "/o_block.png")?,
            s_block_image: graphics::Image::new(ctx, "/s_block.png")?,
            t_block_image: graphics::Image::new(ctx, "/t_block.png")?,
            z_block_image: graphics::Image::new(ctx, "/z_block.png")?,
            b_block_image: graphics::Image::new(ctx, "/b_block.png")?,
            font: Font::new(ctx, "/PressStart2P-Regular.ttf")?,
            theme: audio::Source::new(ctx, "/Tetris_theme.ogg")?,
        })
    }

    pub(crate) fn block_image(&mut self, code: u8) -> Option<&mut Image> {
        match TetriminoType::from_code(code) {
            Some(tetrimino_type) => {
                match tetrimino_type {
                    TetriminoType::I => Option::from(&mut self.i_block_image),
                    TetriminoType::J => Option::from(&mut self.j_block_image),
                    TetriminoType::L => Option::from(&mut self.l_block_image),
                    TetriminoType::O => Option::from(&mut self.o_block_image),
                    TetriminoType::S => Option::from(&mut self.s_block_image),
                    TetriminoType::T => Option::from(&mut self.t_block_image),
                    TetriminoType::Z => Option::from(&mut self.z_block_image),
                }
            }
            None => {
                match BoardType::from_code(code) {
                    None => Option::None,
                    Some(board_type) => {
                        match board_type {
                            BoardType::LIMIT => Option::from(&mut self.b_block_image),
                            _ => Option::None,
                        }
                    }
                }
            }
        }
    }
}
