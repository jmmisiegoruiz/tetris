use ggez::*;
use ggez::graphics::{BLACK, Text, DrawParam, TextFragment};
use ggez::event::{KeyMods, KeyCode};

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);
        let mut text = Text::default();
        text.add(TextFragment::from(format!("Hello ggez! dt = {}ns", self.dt.subsec_nanos())));
        graphics::draw(ctx, &text, DrawParam::default());
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut text = Text::default();
        match keycode {
            KeyCode::Left => {
                println!("Left key pressed");
                text.add(TextFragment::from("Left key pressed"));
                graphics::draw(ctx, &text, DrawParam::default().offset);
            }
            KeyCode::Right => {
                println!("Right key pressed");
                text.add(TextFragment::from("Right key pressed"));
                graphics::draw(ctx, &text, DrawParam::default());
            }
            _ => ()
        }
    }
}

fn main() {
    let state = &mut State {
        dt: std::time::Duration::new(0, 0)
    };

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Just another Tetris");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("tetris", "Jose Matias Misiego Ruiz")
        .conf(config)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
