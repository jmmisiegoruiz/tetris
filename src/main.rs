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
use ggez::audio::SoundSource;
use crate::experimental::new_executor_and_spawner;
use crate::timer::TimerFuture;
use std::time::Duration;

mod scenes;
mod world;
mod constants;
mod types;
mod drawing;
mod resources;
mod experimental;
mod timer;

pub struct SharedState {
    game_started: bool,
    assets: Assets,
}

impl SharedState {
    fn new(ctx: &mut Context) -> GameResult<SharedState> {
        let assets = Assets::new(ctx)?;

        let s = SharedState {
            game_started: false,
            assets,
        };

        Ok(s)
    }
}

struct MainState {
    scenes: SceneStack
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut shared_state = SharedState::new(ctx)?;
        shared_state.assets.theme.set_repeat(true);
        shared_state.assets.theme.play()?;

        let mut main_state = MainState {
            scenes: SceneStack::new(ctx, shared_state)
        };
        main_state.scenes.push(game_over::GameOverScene::new()?);
        main_state.scenes.push(game_play::GamePlayScene::new(ctx)?);
        main_state.scenes.push(start::StartScene::new()?);
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
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    spawner.spawn(async {
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

        let game = &mut MainState::new(ctx).unwrap();

        event::run(ctx, event_loop, game).unwrap();
    });

    drop(spawner);

    executor.run();

    Ok(())
}
