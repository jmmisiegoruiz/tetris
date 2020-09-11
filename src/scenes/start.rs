use ggez::{GameResult, Context, graphics};
use ggez_goodies::scene::{Scene, SceneSwitch};
use ggez::event::KeyCode;
use ggez::graphics::{Text, BLACK};
use crate::SharedState;
use crate::types::ScreenPoint2;

pub struct StartScene {
    start: bool
}

impl StartScene {
    pub fn new() -> GameResult<Box<StartScene>> {
        let start_scene = StartScene {
            start: false
        };
        Ok(Box::new(start_scene))
    }
}

impl Scene<SharedState, KeyCode> for StartScene {
    fn update(&mut self, _shared_state: &mut SharedState, _ctx: &mut Context) -> SceneSwitch<SharedState, KeyCode> {
        if self.start {
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, _shared_state: &mut SharedState, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        let some_text = Text::new("START SCENE");

        graphics::draw(
            ctx,
            &some_text,
            (ScreenPoint2::new(100.0,100.0), graphics::WHITE)
        )?;

        graphics::present(ctx)
    }

    fn input(&mut self, _shared_state: &mut SharedState, event: KeyCode, _started: bool) {
        match event {
            KeyCode::Space => {
                self.start = true;
            }
            _ => ()
        }
    }

    fn name(&self) -> &str {
        "StartScene"
    }

    fn draw_previous(&self) -> bool {
        false
    }
}
