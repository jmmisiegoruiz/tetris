use ggez::*;
use ggez::graphics::{BLACK, Text, DrawParam, TextFragment, Mesh, DrawMode, Rect, Color, WHITE};
use ggez::event::{KeyMods, KeyCode};
use ggez::nalgebra as na;

type Point2 = na::Point2<f32>;

#[derive(Debug)]
enum ActorType {
    Block,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2,
}

fn create_block() -> Actor {
    Actor {
        tag: ActorType::Block,
        pos: Point2::origin(),
    }
}

struct State {
    block: Actor,
    dt: std::time::Duration,
    pos_x: f32,
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

        let square = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::from([self.pos_x, 0.0, 50.0, 50.0]),
            WHITE
        )?;

        graphics::draw(ctx, &square, (na::Point2::new(100.0, 0.0),));

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut text = Text::default();
        match keycode {
            KeyCode::Left => {
                println!("Left key pressed");
                text.add(TextFragment::from("Left key pressed"));
                graphics::draw(ctx, &text, (na::Point2::new(200.0, 200.0),));
            }
            KeyCode::Right => {
                println!("Right key pressed");
                text.add(TextFragment::from("Right key pressed"));
                graphics::draw(ctx, &text, DrawParam::default());

                self.pos_x = self.pos_x % 800.0 + 1.0;
            }
            _ => ()
        }
    }
}

fn main() {
    let state = &mut State {
        dt: std::time::Duration::new(0, 0),
        pos_x: 0.0
    };

    let mut config = conf::Conf::new();
    config.window_setup.title = String::from("Just another Tetris");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("tetris", "Jose Matias Misiego Ruiz")
        .conf(config)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
