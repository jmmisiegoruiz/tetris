use ggez::event::{KeyMods, KeyCode};
use ggez::{GameResult, Context, ContextBuilder, conf, event, timer, graphics};
use std::env;
use std::path;
use crate::resources::Assets;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT, DESIRED_FPS, SCALE_FACTOR};
use ggez::audio::SoundSource;
use specs::*;
use ecs::resources::Motion;
use crate::ecs::systems::{MovementSystem, CollisionSystem};
use crate::ecs::components::{Position, ControllableTag, Image, CollisionBox, Collision};
use mint;
use crate::world::{TetriminoType, Rotation, Board};
use mint::Point2;
use ggez_goodies::input::InputState;
use crate::inputs::{Axes, Buttons, make_input_binding};
use ggez::graphics::Text;
use ggez::nalgebra as na;
use crate::ecs::resources::Timing;

mod ecs;
mod scenes;
mod world;
mod constants;
mod types;
mod drawing;
mod resources;
mod inputs;

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
    dt: std::time::Duration,
    specs_world: World,
    movement_system: MovementSystem,
    collision_system: CollisionSystem,
    input_state: InputState<Axes, Buttons>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut shared_state = SharedState::new(ctx)?;
        shared_state.assets.theme.set_repeat(true);
        shared_state.assets.theme.play()?;

        let dt = std::time::Duration::new(0, 0);

        let mut world = World::new();
        world.register::<Position>();
        world.register::<Image>();
        world.register::<ControllableTag>();
        world.register::<CollisionBox>();
        world.register::<Collision>();

        world
            .create_entity()
            .with(Position {
                position: mint::Point2::from([0.0, -120.0]),
            })
            .with(
                TetriminoType::image(
                    &TetriminoType::T,
                    ctx,
                    0.0, SCALE_FACTOR)?)
            .with(TetriminoType::collision_box(&TetriminoType::T, Rotation::Zero).unwrap())
            .build();

        world
            .create_entity()
            .with(Position {
                position: mint::Point2::from([0.0, 0.0]),
            })
            .with(
                TetriminoType::image(
                    &TetriminoType::O,
                    ctx,
                    0.0, SCALE_FACTOR)?)
            .with(TetriminoType::collision_box(&TetriminoType::O, Rotation::Zero).unwrap())
            .with(ControllableTag)
            .build();

        let board = Board::new();

        world
            .create_entity()
            .with(Position {
                position: mint::Point2::from([0.0, -240.0]),
            })
            .with(
                board.image(
                    ctx,
                    0.0, SCALE_FACTOR)?)
            .with(board.collision_box()?)
            .build();

        make_input_binding();
        world.insert(Motion::default());
        world.insert(Timing::default());

        let update_pos = MovementSystem;
        let detect_collisions = CollisionSystem;

        let ms = MainState {
            dt,
            specs_world: world,
            movement_system: update_pos,
            collision_system: detect_collisions,
            input_state: InputState::new(),
        };

        Ok(ms)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.dt = timer::delta(ctx);

            //println!("dt = {}ns", self.dt.subsec_nanos());
            //println!("fps = {}", timer::fps(ctx));

            // run our update systems here
            self.collision_system.run_now(&self.specs_world);
            self.movement_system.run_now(&self.specs_world);

            self.specs_world.maintain();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Get the components we need from the world for drawing
        let positions = self.specs_world.read_storage::<Position>();
        let images = self.specs_world.read_storage::<Image>();
        let collisions = self.specs_world.read_storage::<Collision>();

        // this is our rendering "system"
        for (p, i) in (&positions, &images).join() {
            graphics::draw(
                ctx,
                &*i.mesh,
                (
                    world_to_screen_coordinates(SCREEN_WIDTH, SCREEN_HEIGHT, p.position),
                    i.rotation,
                    i.offset,
                    i.scale,
                    i.color
                ),
            ).unwrap_or_else(|err| println!("draw error {:?}", err));

            let position = Text::new(format!("({}, {})", p.position.x, p.position.y));

            graphics::draw(
                ctx,
                &position,
                (
                    world_to_screen_coordinates(SCREEN_WIDTH, SCREEN_HEIGHT, p.position),
                    graphics::WHITE),
            )?;
        }

        for collision in (&collisions).join() {
            let depth = Text::new(format!("depth: {:?}", collision.contact.depth));

            graphics::draw(
                ctx,
                &depth,
                (na::Point2::new(0.0, 500.0), graphics::WHITE),
            )?;
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            // we don't multiple registrations of a keypress
            match keycode {
                KeyCode::Up => {
                    self.input_state.update_axis_start(Axes::Vertical, true);
                }
                KeyCode::Down => {
                    self.input_state.update_axis_start(Axes::Vertical, false);
                }
                KeyCode::Left => {
                    self.input_state.update_axis_start(Axes::Horizontal, false);
                }
                KeyCode::Right => {
                    self.input_state.update_axis_start(Axes::Horizontal, true);
                }
                KeyCode::Space => {
                    self.input_state.update_button_down(Buttons::RotateRight)
                }
                _ => (),
            }
            // Update the world-owned player_input struct to match the current
            // state of the MainState owned struct
            let mut input_state = self.specs_world.write_resource::<Motion>();
            *input_state = Motion::new(&self.input_state);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up => {
                self.input_state.update_axis_stop(Axes::Vertical, true);
            }
            KeyCode::Down => {
                self.input_state.update_axis_stop(Axes::Vertical, false);
            }
            KeyCode::Left => {
                self.input_state.update_axis_stop(Axes::Horizontal, false);
            }
            KeyCode::Right => {
                self.input_state.update_axis_stop(Axes::Horizontal, true);
            }
            KeyCode::Space => {
                self.input_state.update_button_up(Buttons::RotateRight)
            }
            _ => (),
        }

        // track the MainState input in the Movement resource in the specs world
        let mut input_state = self.specs_world.write_resource::<Motion>();
        *input_state = Motion::new(&self.input_state);
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

fn world_to_screen_coordinates(screen_width: f32, screen_height: f32, world_point: mint::Point2<f32>) -> mint::Point2<f32>{
    Point2 {
        x: world_point.x + screen_width/2.0,
        y: -world_point.y + screen_height/2.0
    }
}
