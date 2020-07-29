use ggez::graphics::{BLACK};
use ggez::event::{KeyMods, KeyCode};
use ggez::{nalgebra as na, GameResult, Context, ContextBuilder, conf, graphics, event, timer};
use std::env;
use std::path;
use ggez::nalgebra::{Matrix2};
use arrayvec::ArrayVec;

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

#[derive(Debug)]
enum Direction {
    DOWN(i8),
    LEFT(i8),
    RIGHT(i8),
}

struct Assets {
    block_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let block_image = graphics::Image::new(ctx, "/block.png")?;

        Ok(Assets {
            block_image,
        })
    }

    fn block_image(&mut self, tetrimino: &Tetrimino) -> &mut graphics::Image {
        match tetrimino.kind {
            TetriminoType::I => &mut self.block_image,
            TetriminoType::J => &mut self.block_image,
            TetriminoType::L => &mut self.block_image,
            TetriminoType::O => &mut self.block_image,
            TetriminoType::S => &mut self.block_image,
            TetriminoType::T => &mut self.block_image,
            TetriminoType::Z => &mut self.block_image,
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
        match kind {
            TetriminoType::I => Tetrimino {
                kind: TetriminoType::I,
                pos: WorldPoint2::new(4, 0),
                vectors: [WorldVector2::new(-1, 0), WorldVector2::new(1, 0), WorldVector2::new(2, 0)],
            },
            TetriminoType::J => Tetrimino {
                kind: TetriminoType::J,
                pos: WorldPoint2::origin(),
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, -1)],
            },
            TetriminoType::L => Tetrimino {
                kind: TetriminoType::L,
                pos: WorldPoint2::origin(),
                vectors: [WorldVector2::new(0, -1), WorldVector2::new(1, 0), WorldVector2::new(2, 0)],
            },
            TetriminoType::O => Tetrimino {
                kind: TetriminoType::O,
                pos: WorldPoint2::origin(),
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, -1)],
            },
            TetriminoType::S => Tetrimino {
                kind: TetriminoType::S,
                pos: WorldPoint2::origin(),
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, -1)],
            },
            TetriminoType::T => Tetrimino {
                kind: TetriminoType::T,
                pos: WorldPoint2::origin(),
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, -1)],
            },
            TetriminoType::Z => Tetrimino {
                kind: TetriminoType::Z,
                pos: WorldPoint2::origin(),
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, -1)],
            },
        }
    }

    fn move_left(&mut self) {
        if self.can_move(Direction::LEFT(0)) {
            self.pos.x += -1;
        }
    }

    fn move_right(&mut self) {
        if self.can_move(Direction::RIGHT(9)) {
            self.pos.x += 1;
        }
    }

    fn move_down(&mut self) {
        if self.can_move(Direction::DOWN(19)) {
            self.pos.y += 1;
        }
    }

    fn can_move(&mut self, direction: Direction) -> bool {
        let mut can_move = true;
        for vector in self.vectors.iter() {

            match direction {
                Direction::DOWN(limit) => {
                    if self.pos.y + vector.y + 1 > limit {
                        can_move = false;
                    }
                },
                Direction::LEFT(limit) => {
                    if self.pos.x + vector.x - 1 < limit {
                        can_move = false;
                    }
                },
                Direction::RIGHT(limit) => {
                    if self.pos.x + vector.x + 1 > limit {
                        can_move = false;
                    }
                },
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
    screen_width: f32,
    screen_height: f32,
    tetrimino: Tetrimino,
    fall_timeout: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let tetrimino = Tetrimino::from(TetriminoType::L);
        let (width, height) = graphics::drawable_size(ctx);

        let s = MainState {
            assets,
            screen_width: width,
            screen_height: height,
            tetrimino,
            fall_timeout: FALL_TIME
        };

        Ok(s)
    }
}

fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: &WorldPoint2) -> ScreenPoint2 {
    let x = (point.x as f32) * (screen_width / 10.0);
    let y = (point.y as f32) * (screen_height / 20.0);
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
    let image = assets.block_image(tetrimino);

    for vector in tetrimino.vectors.iter() {
        let vector_pos =
            world_to_screen_coords(
                screen_w,
                screen_h,
                &WorldPoint2::from([vector.x + tetrimino.pos.x, vector.y + tetrimino.pos.y]));
        let draw_params = graphics::DrawParam::new()
            .dest(vector_pos);
        graphics::draw(ctx, image, draw_params);
    }

    let draw_params = graphics::DrawParam::new()
        .dest(pos);
    graphics::draw(ctx, image, draw_params)
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            self.fall_timeout -= seconds;
            if self.fall_timeout < 0.0 {
                self.tetrimino.move_down();
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

            let tetrimino = &self.tetrimino;
            draw_tetrimino(assets, ctx, tetrimino, coords)?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => {
                self.tetrimino.move_left();
            }
            KeyCode::Right => {
                self.tetrimino.move_right();
            }
            KeyCode::Down => {
                self.tetrimino.move_down();
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
    config.window_mode.width = 440.0;
    config.window_mode.height = 880.0;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("tetris", "Jose Matias Misiego Ruiz")
        .conf(config)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let game = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, game)
}
