use ggez::{GameResult, Context, graphics};
use ggez_goodies::scene::{Scene, SceneSwitch};
use ggez::event::{KeyCode, quit};
use ggez::graphics::{Text, BLACK};
use crate::SharedState;
use crate::types::ScreenPoint2;

pub struct GameOverScene {
    quit: bool
}

impl GameOverScene {
    pub fn new() -> GameResult<Box<GameOverScene>> {
        let game_over_scene = GameOverScene {
            quit: false
        };
        Ok(Box::new(game_over_scene))
    }
}

impl Scene<SharedState, KeyCode> for GameOverScene {
    fn update(&mut self, _shared_state: &mut SharedState, ctx: &mut Context) -> SceneSwitch<SharedState, KeyCode> {
        if self.quit {
            quit(ctx);
        }
        SceneSwitch::None
    }

    fn draw(&mut self, _shared_state: &mut SharedState, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        let some_text = Text::new("GAME OVER SCENE");

        graphics::draw(
            ctx,
            &some_text,
            (ScreenPoint2::new(100.0,100.0), graphics::WHITE)
        )?;

        graphics::present(ctx)
    }

    fn input(&mut self, _shared_state: &mut SharedState, event: KeyCode, _started: bool) {
        match event {
            KeyCode::Escape => {
                self.quit = true;
            }
            _ => ()
        }
    }

    fn name(&self) -> &str {
        "GameOverScene"
    }

    fn draw_previous(&self) -> bool {
        false
    }
}
