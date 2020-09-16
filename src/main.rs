use ggez::event::{KeyMods, KeyCode};
use ggez::{GameResult, Context, ContextBuilder, conf, event};
use std::env;
use std::path;
use scenes::start;
use scenes::game_over;
use scenes::game_play;
use types::SceneStack;
use crate::resources::Assets;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

mod scenes;
mod world;
mod constants;
mod types;
mod drawing;
mod resources;

pub struct SharedState {
    game_started: bool,
    assets: Assets
}

impl SharedState {
    fn new(ctx: &mut Context) -> GameResult<SharedState> {
        let assets = Assets::new(ctx)?;

        let s = SharedState {
            game_started: false,
            assets
        };

        Ok(s)
    }
}

struct MainState {
    scenes: SceneStack
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let shared_state = SharedState::new(ctx)?;
        let mut main_state = MainState {
            scenes: SceneStack::new(ctx, shared_state)
        };
        main_state.scenes.push(game_over::GameOverScene::new()?);
        main_state.scenes.push(game_play::GamePlayScene::new()?);
        main_state.scenes.push(start::StartScene::new(ctx)?);
        Ok(main_state)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scenes.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scenes.draw(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.scenes.input(keycode, false)
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
    config.window_mode.width = SCREEN_WIDTH;
    config.window_mode.height = SCREEN_HEIGHT;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("tetris", "Jose Matias Misiego Ruiz")
        .conf(config)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let game = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, game)
}
