use ggez::{GameResult, Context, graphics, audio};
use ggez_goodies::scene::{Scene, SceneSwitch};
use ggez::event::KeyCode;
use ggez::graphics::{Text, BLACK, Scale, Font, TextFragment, Color, Drawable, DrawParam};
use crate::SharedState;
use crate::types::ScreenPoint2;
use ggez::timer::TimeContext;
use ggez::audio::SoundSource;

pub struct StartScene {
    theme: audio::Source
}

impl StartScene {
    pub fn new(ctx: &mut Context) -> GameResult<Box<StartScene>> {
        let mut theme=  audio::Source::new(ctx, "/Tetris_theme.ogg")?;
        theme.set_repeat(true);
        theme.play()?;

        let start_scene = StartScene {
            theme
        };
        Ok(Box::new(start_scene))
    }
}

impl Scene<SharedState, KeyCode> for StartScene {
    fn update(&mut self, shared_state: &mut SharedState, _ctx: &mut Context) -> SceneSwitch<SharedState, KeyCode> {
        if shared_state.game_started {
            _ctx.timer_context = TimeContext::new();
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, _shared_state: &mut SharedState, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        let font = Some(Font::new(ctx, "/PressStart2P-Regular.ttf").unwrap());
        let scale = Some(Scale {x: 100.0, y: 300.0});
        let t_fragment = TextFragment {
            text: "T".to_string(),
            color: Some(Color::from_rgb(156, 16, 229)),
            font,
            scale,
        };

        let e_fragment = TextFragment {
            text: "E".to_string(),
            color: Some(Color::from_rgb(0, 229, 37)),
            font,
            scale,
        };

        let tt_fragment = TextFragment {
            text: "T".to_string(),
            color: Some(Color::from_rgb(156, 16, 229)),
            font,
            scale,
        };

        let r_fragment = TextFragment {
            text: "R".to_string(),
            color: Some(Color::from_rgb(230, 0, 36)),
            font,
            scale,
        };

        let i_fragment = TextFragment {
            text: "I".to_string(),
            color: Some(Color::from_rgb(229, 223, 36)),
            font,
            scale,
        };

        let s_fragment = TextFragment {
            text: "S".to_string(),
            color: Some(Color::from_rgb(0, 229, 37)),
            font,
            scale,
        };

        Text::default()
            .add(t_fragment)
            .add(e_fragment)
            .add(tt_fragment)
            .add(r_fragment)
            .add(i_fragment)
            .add(s_fragment)
            .draw(ctx, DrawParam::from((ScreenPoint2::new(100.0, 100.0), )))?;

        Text::new("PRESS SPACE TO START")
            .set_font(font.unwrap(), Scale::uniform(20.0))
            .draw(ctx, DrawParam::from((ScreenPoint2::new(200.0, 450.0), )))?;

        Text::new("CONTROLS: ARROW KEYS + W/Q FOR ROTATING CLOCKWISE/COUNTER CLOCKWISE")
            .set_font(font.unwrap(), Scale::uniform(10.0))
            .draw(ctx, DrawParam::from((ScreenPoint2::new(50.0, 500.0), )))?;

        graphics::present(ctx)
    }

    fn input(&mut self, shared_state: &mut SharedState, event: KeyCode, _started: bool) {
        match event {
            KeyCode::Space => {
                shared_state.game_started = true;
                self.theme.stop();
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
