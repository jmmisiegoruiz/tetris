use ggez::{GameResult, Context, graphics, timer};
use ggez_goodies::scene::{Scene, SceneSwitch};
use ggez::event::KeyCode;
use rand::Rng;
use num_traits::ToPrimitive;
use crate::SharedState;
use crate::constants::{BOARD_WIDTH, BOARD_HEIGHT, FALL_TIME};
use crate::world::{Board, TetriminoType, Tetrimino, ScoreBoard};
use crate::drawing::{draw_tetrimino, draw_board, draw_score_board};
use ggez::graphics::BLACK;

struct GamePlayState {
    board: Board,
    fall_timeout: f32,
    next_tetrimino: TetriminoType,
    score: ScoreBoard,
    tetrimino: Tetrimino,
}

impl GamePlayState {
    fn new() -> GameResult<GamePlayState> {
        let board = Board::new();
        let mut rng = rand::thread_rng();
        let tetrimino = Tetrimino::from(&TetriminoType::from_code(rng.gen_range(1, 8)).unwrap());
        let next_tetrimino = TetriminoType::from_code(rng.gen_range(1, 8)).unwrap();
        let score = ScoreBoard::new();

        let game_play_state = GamePlayState {
            board,
            fall_timeout: FALL_TIME,
            next_tetrimino,
            score,
            tetrimino,
        };

        Ok(game_play_state)
    }
}

pub struct GamePlayScene {
    state: GamePlayState
}

impl GamePlayScene {
    pub fn new() -> GameResult<Box<GamePlayScene>> {
        let game_play_scene = GamePlayScene {
            state: GamePlayState::new()?
        };
        Ok(Box::new(game_play_scene))
    }
}

impl Scene<SharedState, KeyCode> for GamePlayScene {
    fn update(&mut self, _game_world: &mut SharedState, ctx: &mut Context) -> SceneSwitch<SharedState, KeyCode> {
        let scene_state = &mut self.state;
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            scene_state.fall_timeout -= seconds;
            if scene_state.fall_timeout < 0.0 {
                let can_move = scene_state.tetrimino.move_down(&scene_state.board);
                if !can_move && scene_state.tetrimino.pos.y == 1 {
                    return SceneSwitch::Pop;
                } else if !can_move {
                    &scene_state.board.update(&scene_state.tetrimino, &mut scene_state.score);
                    let mut rng = rand::thread_rng();
                    scene_state.tetrimino = Tetrimino::from(&scene_state.next_tetrimino);
                    scene_state.next_tetrimino = TetriminoType::from_code(rng.gen_range(1, 8)).unwrap();
                }
                scene_state.fall_timeout = FALL_TIME / (scene_state.score.level + 1).to_f32().unwrap();
            }
        }
        SceneSwitch::None
    }

    fn draw(&mut self, shared_state: &mut SharedState, ctx: &mut Context) -> GameResult<()> {
        let scene_state = &mut self.state;
        graphics::clear(ctx, BLACK);

        {
            let assets = &mut shared_state.assets;
            let board_dimensions = (scene_state.board.width, scene_state.board.height);

            draw_tetrimino(assets, ctx, &scene_state.tetrimino, board_dimensions, None)?;
            draw_tetrimino(assets, ctx, &Tetrimino::from(&scene_state.next_tetrimino), board_dimensions, Option::from((BOARD_WIDTH, BOARD_HEIGHT / 2.0)))?;
            draw_board(assets, ctx, &scene_state.board, board_dimensions)?;
            draw_score_board(ctx, &scene_state.score)?;
        }

        graphics::present(ctx)
    }

    fn input(&mut self, _game_world: &mut SharedState, event: KeyCode, _started: bool) {
        let scene_state = &mut self.state;

        match event {
            KeyCode::Left => {
                scene_state.tetrimino.move_left(&scene_state.board);
            }
            KeyCode::Right => {
                scene_state.tetrimino.move_right(&scene_state.board);
            }
            KeyCode::Down => {
                scene_state.tetrimino.move_down(&scene_state.board);
            }
            KeyCode::Q => {
                scene_state.tetrimino.rotate_counter_clockwise();
            }
            KeyCode::W => {
                scene_state.tetrimino.rotate_clockwise();
            }
            _ => ()
        }
    }

    fn name(&self) -> &str {
        "GamePlayScene"
    }

    fn draw_previous(&self) -> bool {
        false
    }
}
