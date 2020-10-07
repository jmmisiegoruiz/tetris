use ggez::{GameResult, Context, graphics, timer, audio};
use ggez_goodies::scene::{Scene, SceneSwitch};
use ggez::event::KeyCode;
use rand::Rng;
use num_traits::ToPrimitive;
use crate::SharedState;
use crate::constants::{BOARD_WIDTH, BOARD_HEIGHT, FALL_TIME, TOP_BOUNDARY};
use crate::world::{Board, TetriminoType, Tetrimino, ScoreBoard};
use crate::drawing::{draw_tetrimino, draw_board, draw_score_board};
use ggez::graphics::{BLACK, Text, Scale};
use ggez::audio::SoundSource;
use crate::types::ScreenPoint2;
use ggez::timer::TimeContext;


struct GamePlayState {
    board: Board,
    fall_timeout: f32,
    game_over: bool,
    start_again:bool,
    next_tetrimino: TetriminoType,
    score: ScoreBoard,
    tetrimino: Tetrimino,
}

impl GamePlayState {
    fn new() -> GameResult<GamePlayState> {
        let board = Board::new();
        let tetrimino = Tetrimino::from(&GamePlayScene::get_random_tetrimino_type().unwrap());
        let next_tetrimino = GamePlayScene::get_random_tetrimino_type().unwrap();
        let score = ScoreBoard::new();

        let game_play_state = GamePlayState {
            board,
            fall_timeout: FALL_TIME,
            game_over: false,
            next_tetrimino,
            score,
            start_again: false,
            tetrimino,
        };

        Ok(game_play_state)
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.board = Board::new();
        self.game_over = false;
        self.score = ScoreBoard::new();
        self.next_tetrimino = GamePlayScene::get_random_tetrimino_type().unwrap();
        self.tetrimino = Tetrimino::from(&GamePlayScene::get_random_tetrimino_type().unwrap());

        ctx.timer_context = TimeContext::new();
    }
}

struct SoundEffects {
    fall: audio::Source,
    game_over: audio::Source,
    line: audio::Source,
}

impl SoundEffects {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let sound_effects = SoundEffects {
            fall: audio::Source::new(ctx, "/fall.ogg")?,
            line: audio::Source::new(ctx, "/line.ogg")?,
            game_over: audio::Source::new(ctx, "/game_over.ogg")?,
        };
        Ok(sound_effects)
    }
}

pub struct GamePlayScene {
    state: GamePlayState,
    sound_effects: SoundEffects
}

impl GamePlayScene {
    pub fn new(ctx: &mut Context) -> GameResult<Box<GamePlayScene>> {
        let game_play_scene = GamePlayScene {
            state: GamePlayState::new()?,
            sound_effects: SoundEffects::new(ctx)?
        };
        Ok(Box::new(game_play_scene))
    }
}

impl Scene<SharedState, KeyCode> for GamePlayScene {
    fn update(&mut self, _shared_state: &mut SharedState, ctx: &mut Context) -> SceneSwitch<SharedState, KeyCode> {
        let scene_state = &mut self.state;

        if scene_state.start_again && scene_state.game_over {
            scene_state.reset(ctx) ;
            scene_state.start_again = false;
            scene_state.game_over = false;
        }

        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            scene_state.fall_timeout -= seconds;
            if scene_state.fall_timeout < 0.0 {
                let can_move = scene_state.tetrimino.move_down(&scene_state.board);
                if !can_move && scene_state.tetrimino.pos.y == TOP_BOUNDARY {
                    if !self.state.game_over {
                        self.state.game_over = true;
                        self.sound_effects.game_over.play().unwrap();
                    }
                    return SceneSwitch::None;
                } else if !can_move {
                    self.sound_effects.fall.play().unwrap();
                    if &scene_state.board.update(&scene_state.tetrimino, &mut scene_state.score) > &0 {
                        self.sound_effects.line.play().unwrap();
                    };
                    scene_state.tetrimino = Tetrimino::from(&scene_state.next_tetrimino);
                    scene_state.next_tetrimino = GamePlayScene::get_random_tetrimino_type().unwrap();
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
            draw_score_board(ctx, &scene_state.score, shared_state)?;

            if scene_state.game_over {
                let mut game_over_text = Text::new("GAME OVER");
                game_over_text.set_font(shared_state.assets.font, Scale::uniform(50.0));
                graphics::draw(
                    ctx,
                    &game_over_text,
                    (ScreenPoint2::new(180.0,30.0), graphics::WHITE)
                )?;
            }
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
                scene_state.tetrimino.rotate_counter_clockwise(&scene_state.board);
            }
            KeyCode::W => {
                scene_state.tetrimino.rotate_clockwise(&scene_state.board);
            }
            KeyCode::Space => {
                if scene_state.game_over {
                    scene_state.start_again = true;
                }
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

impl GamePlayScene {
    fn get_random_tetrimino_type() -> Option<TetriminoType> {
        let mut rng = rand::thread_rng();
        TetriminoType::from_code(rng.gen_range(1, 8))
    }
}
