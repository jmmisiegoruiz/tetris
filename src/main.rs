use ggez::graphics::{BLACK, Image};
use ggez::event::{KeyMods, KeyCode};
use ggez::{nalgebra as na, GameResult, Context, ContextBuilder, conf, graphics, event, timer};
use std::env;
use std::path;
use ggez::nalgebra::{Matrix2, MatrixMN, U21, U12, U20, U10};
use arrayvec::ArrayVec;
use rand;
use std::convert::TryFrom;
use rand::Rng;

type ScreenPoint2 = na::Point2<f32>;
type WorldPoint2 = na::Point2<i8>;
type WorldVector2 = na::Vector2<i8>;

#[derive(Debug)]
enum TetriminoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetriminoType {
    fn to_code(&self) -> u8 {
        match self {
            TetriminoType::I => 1,
            TetriminoType::J => 2,
            TetriminoType::L => 3,
            TetriminoType::O => 4,
            TetriminoType::S => 5,
            TetriminoType::T => 6,
            TetriminoType::Z => 7,
        }
    }

    fn from_code(code: u8) -> Option<TetriminoType> {
        match code {
            1 => Option::from(TetriminoType::I),
            2 => Option::from(TetriminoType::J),
            3 => Option::from(TetriminoType::L),
            4 => Option::from(TetriminoType::O),
            5 => Option::from(TetriminoType::S),
            6 => Option::from(TetriminoType::T),
            7 => Option::from(TetriminoType::Z),
            _ => Option::None,
        }
    }
}

#[derive(Debug)]
enum BoardType {
    EMPTY,
    BLOCK,
    LIMIT,
}

impl BoardType {
    fn from_code(code: u8) -> Option<BoardType>{
        match code {
            0 => Option::from(BoardType::EMPTY),
            99 => Option::from(BoardType::LIMIT),
            _ => Option::from(BoardType::BLOCK),
        }
    }
}

#[derive(Debug)]
enum Direction {
    DOWN,
    LEFT,
    RIGHT,
}

struct Assets {
    i_block_image: graphics::Image,
    j_block_image: graphics::Image,
    l_block_image: graphics::Image,
    o_block_image: graphics::Image,
    s_block_image: graphics::Image,
    t_block_image: graphics::Image,
    z_block_image: graphics::Image,
    b_block_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            i_block_image: graphics::Image::new(ctx, "/i_block.png")?,
            j_block_image: graphics::Image::new(ctx, "/j_block.png")?,
            l_block_image: graphics::Image::new(ctx, "/l_block.png")?,
            o_block_image: graphics::Image::new(ctx, "/o_block.png")?,
            s_block_image: graphics::Image::new(ctx, "/s_block.png")?,
            t_block_image: graphics::Image::new(ctx, "/t_block.png")?,
            z_block_image: graphics::Image::new(ctx, "/z_block.png")?,
            b_block_image: graphics::Image::new(ctx, "/b_block.png")?,
        })
    }

    fn block_image(&mut self, code: u8) -> Option<&mut Image> {
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
            },
            None => {
                match BoardType::from_code(code) {
                    None => Option::None,
                    Some(board_type) => {
                        match board_type {
                            BoardType::LIMIT => Option::from(&mut self.b_block_image),
                            _ => Option::None,
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug)]
struct Board {
    data: MatrixMN<u8, U21, U12>
}

impl Board {
    fn new() -> Board {
        let data: MatrixMN<u8, U21, U12> = MatrixMN::<u8, U20, U10>::zeros()
            .insert_row(20, 99)
            .insert_column(0, 99)
            .insert_column(11, 99);
        Board {
            data
        }
    }

    fn update(&mut self, tetrimino: &Tetrimino) {
        *self.data.index_mut(to_matrix_index(tetrimino.pos.x, tetrimino.pos.y)) = tetrimino.kind.to_code();
        for vector in tetrimino.vectors.iter() {
            *self.data.index_mut(to_matrix_index(tetrimino.pos.x + vector.x, tetrimino.pos.y + vector.y)) = tetrimino.kind.to_code();
        }
    }
}

#[derive(Debug)]
struct Tetrimino {
    kind: TetriminoType,
    pos: WorldPoint2,
    vectors: [WorldVector2; 3],
}

impl Tetrimino {
    const CLOCKWISE_MATRIX: [i8; 4] = [0, -1, 1, 0];
    const COUNTER_CLOCKWISE_MATRIX: [i8; 4] = [0, 1, -1, 0];

    fn from(kind: TetriminoType) -> Tetrimino {
        let pos = WorldPoint2::new(4, 1);
        match kind {
            TetriminoType::I => Tetrimino {
                kind: TetriminoType::I,
                pos,
                vectors: [WorldVector2::new(-1, 0), WorldVector2::new(1, 0), WorldVector2::new(2, 0)],
            },
            TetriminoType::J => Tetrimino {
                kind: TetriminoType::J,
                pos,
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, 1)],
            },
            TetriminoType::L => Tetrimino {
                kind: TetriminoType::L,
                pos,
                vectors: [WorldVector2::new(0, 1), WorldVector2::new(1, 0), WorldVector2::new(2, 0)],
            },
            TetriminoType::O => Tetrimino {
                kind: TetriminoType::O,
                pos,
                vectors: [WorldVector2::new(1, 0), WorldVector2::new(0, 1), WorldVector2::new(1, 1)],
            },
            TetriminoType::S => Tetrimino {
                kind: TetriminoType::S,
                pos,
                vectors: [WorldVector2::new(-1, 1), WorldVector2::new(0, 1), WorldVector2::new(1, 0)],
            },
            TetriminoType::T => Tetrimino {
                kind: TetriminoType::T,
                pos,
                vectors: [WorldVector2::new(-1, 0), WorldVector2::new(0, 1), WorldVector2::new(1, 0)],
            },
            TetriminoType::Z => Tetrimino {
                kind: TetriminoType::Z,
                pos,
                vectors: [WorldVector2::new(-1, -1), WorldVector2::new(0, -1), WorldVector2::new(1, 0)],
            },
        }
    }

    fn move_left(&mut self, board: &Board) {
        if self.can_move(Direction::LEFT, board) {
            self.pos.x += -1;
        }
    }

    fn move_right(&mut self, board: &Board) {
        if self.can_move(Direction::RIGHT, board) {
            self.pos.x += 1;
        }
    }

    fn move_down(&mut self, board: &Board) -> bool {
        let can_move = self.can_move(Direction::DOWN, board);
        if can_move {
            self.pos.y += 1;
        }
        can_move
    }

    fn can_move(&mut self, direction: Direction, board: &Board) -> bool {
        let mut can_move = true;
        for (idx, vector) in self.vectors.iter().enumerate() {
            println!("Vector: {}", vector);

            match direction {
                Direction::DOWN => {
                    if let Some(value) = board.data.get(to_matrix_index(
                        self.pos.x + vector.x,
                        self.pos.y + vector.y + 1,
                    )) {
                        println!("Board value {} at index: ({}, {}) for vector {}", value, self.pos.y + vector.y + 1, self.pos.x + vector.x, idx);
                        if value > &0 {
                            can_move = false;
                        }
                    }
                }
                Direction::LEFT => {
                    if let Some(value) = board.data.get(to_matrix_index(
                        self.pos.x + vector.x - 1,
                        self.pos.y + vector.y,
                    )) {
                        println!("Board value {} at index: ({}, {}) for vector {}", value, self.pos.y + vector.y, self.pos.x + vector.x - 1, idx);
                        if value > &0 {
                            can_move = false;
                        }
                    }
                }
                Direction::RIGHT => {
                    if let Some(value) = board.data.get(to_matrix_index(
                        self.pos.x + vector.x + 1,
                        self.pos.y + vector.y,
                    )) {
                        println!("Board value {} at index: ({}, {}) for vector {}", value, self.pos.y + vector.y, self.pos.x + vector.x + 1, idx);
                        if value > &0 {
                            can_move = false;
                        }
                    }
                }
            }
            if !can_move {
                break;
            }
        }
        can_move
    }

    fn move_up(&mut self) {
        self.pos.y += -1;
    }

    fn rotate_counter_clockwise(&mut self) {
        let rotation_matrix = Matrix2::from_row_slice(&Tetrimino::COUNTER_CLOCKWISE_MATRIX);
        let mut new_vectors = ArrayVec::<[WorldVector2; 3]>::new();

        for vector in self.vectors.iter() {
            let result = rotation_matrix * vector;
            new_vectors.push(result);
        }
        self.vectors = new_vectors.into_inner().unwrap();
    }

    fn rotate_clockwise(&mut self) {
        let rotation_matrix = Matrix2::from_row_slice(&Tetrimino::CLOCKWISE_MATRIX);
        let mut new_vectors = ArrayVec::<[WorldVector2; 3]>::new();

        for vector in self.vectors.iter() {
            let result = rotation_matrix * vector;
            new_vectors.push(result);
        }
        self.vectors = new_vectors.into_inner().unwrap();
    }
}

const FALL_TIME: f32 = 5.0;

struct MainState {
    assets: Assets,
    board: Board,
    screen_width: f32,
    screen_height: f32,
    tetrimino: Tetrimino,
    fall_timeout: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let board = Board::new();
        let mut rng = rand::thread_rng();
        let tetrimino = Tetrimino::from(TetriminoType::from_code(rng.gen_range(1, 8)).unwrap());
        let (width, height) = graphics::drawable_size(ctx);

        let s = MainState {
            assets,
            board,
            screen_width: width,
            screen_height: height,
            tetrimino,
            fall_timeout: FALL_TIME,
        };

        Ok(s)
    }
}

fn to_matrix_index(x: i8, y: i8) -> (usize, usize) {
    (usize::try_from(y).expect("Failed to convert Y coordinate"),
     usize::try_from(x).expect("Failed to convert X coordinate"))
}

fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: &WorldPoint2) -> ScreenPoint2 {
    let x = (point.x as f32) * (screen_width / 12.0);
    let y = (point.y as f32) * (screen_height / 21.0);
    ScreenPoint2::new(x, y)
}

fn draw_tetrimino(
    assets: &mut Assets,
    ctx: &mut Context,
    tetrimino: &Tetrimino,
    world_coords: (f32, f32),
) -> GameResult {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, &tetrimino.pos);
    let image = assets.block_image(TetriminoType::to_code(&tetrimino.kind)).unwrap();

    for vector in tetrimino.vectors.iter() {
        let vector_pos =
            world_to_screen_coords(
                screen_w,
                screen_h,
                &WorldPoint2::from([vector.x + tetrimino.pos.x, vector.y + tetrimino.pos.y]));
        let draw_params = graphics::DrawParam::new()
            .scale([0.5,0.5])
            .dest(vector_pos);
        graphics::draw(ctx, image, draw_params)?
    }

    let draw_params = graphics::DrawParam::new()
        .scale([0.5,0.5])
        .dest(pos);
    graphics::draw(ctx, image, draw_params)
}

fn draw_board(
    assets: &mut Assets,
    ctx: &mut Context,
    board: &Board,
    world_coords: (f32, f32),
) -> GameResult {
    let (screen_w, screen_h) = world_coords;
    for (r, row) in board.data.row_iter().enumerate() {
        for (c, _element) in row.column_iter().enumerate() {
            let point =
                world_to_screen_coords(
                    screen_w,
                    screen_h,
                    &WorldPoint2::from([
                        i8::try_from(c).expect("Failed to convert X coordinate"),
                        i8::try_from(r).expect("Failed to convert Y coordinate"),
                    ]));
            let image = assets.block_image(*_element.get((0,0)).unwrap());
            if let Some(image) = image {
                let draw_params = graphics::DrawParam::new()
                    .scale([0.5,0.5])
                    .dest(point);
                graphics::draw(ctx, image, draw_params)?
            }
        }
    }
    GameResult::Ok(())
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.fall_timeout -= seconds;
            if self.fall_timeout < 0.0 {
                if !self.tetrimino.move_down(&self.board) {
                    &self.board.update(&self.tetrimino);
                    let mut rng = rand::thread_rng();
                    self.tetrimino = Tetrimino::from(TetriminoType::from_code(rng.gen_range(1, 8)).unwrap());
                }
                self.fall_timeout = FALL_TIME;
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            draw_tetrimino(assets, ctx, &self.tetrimino, coords)?;
            draw_board(assets, ctx, &self.board, coords)?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => {
                self.tetrimino.move_left(&self.board);
            }
            KeyCode::Right => {
                self.tetrimino.move_right(&self.board);
            }
            KeyCode::Down => {
                self.tetrimino.move_down(&self.board);
            }
            KeyCode::Up => {
                self.tetrimino.move_up();
            }
            KeyCode::Q => {
                self.tetrimino.rotate_counter_clockwise();
            }
            KeyCode::W => {
                self.tetrimino.rotate_clockwise();
            }
            _ => ()
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Just another Tetris");
    config.window_mode.width = 264.0;
    config.window_mode.height = 462.0;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("tetris", "Jose Matias Misiego Ruiz")
        .conf(config)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let game = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, game)
}
