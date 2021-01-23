use ggez::event::{KeyMods, KeyCode, EventHandler};
use ggez::{GameResult, Context, ContextBuilder, conf, event, graphics, timer};
use std::env;
use std::path;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT, DESIRED_FPS};
use mint;
use ggez_goodies::input::InputState;
use crate::inputs::{Axes, Buttons, make_input_binding};
use ncollide2d::na::{Point2};
use ggez::graphics::{DrawParam, BLACK, DrawMode, Rect, Mesh, Color, Text, Scale};
use specs::{World, WorldExt, Join, RunNow, ReadStorage};
use crate::ecs::components::{Position, Image, CollisionObjectHandle, ControllableTag, Collision, StaticTag, LineTag, GameEntityType, BlockTag};
use crate::ecs::utils::create_entity;
use crate::collisions::{CollisionWorldWrapper};
use crate::ecs::systems::{MovementSystem, CollisionsSystem, ControlSystem};
use crate::ecs::resources::{Motion, Timing};
use specs::shred::Fetch;
use ncollide2d::pipeline::CollisionObjectRef;
use crate::drawing::world_to_screen_coordinates;
use std::ops::Deref;
use num_traits::FromPrimitive;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod ecs;
mod constants;
mod inputs;
mod drawing;
mod collisions;

fn main() -> GameResult {
    pretty_env_logger::init();

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

    let game = &mut MainState::new()?;

    event::run(ctx, event_loop, game)?;

    Ok(())
}

pub struct MainState {
    input_state: InputState<Axes, Buttons>,
    specs_world: World,
    movement_system: MovementSystem,
    collisions_system: CollisionsSystem,
    control_system: ControlSystem,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut specs_world = World::new();
        specs_world.register::<Position>();
        specs_world.register::<Image>();
        specs_world.register::<CollisionObjectHandle>();
        specs_world.register::<ControllableTag>();
        specs_world.register::<Collision>();
        specs_world.register::<StaticTag>();
        specs_world.register::<LineTag>();
        specs_world.register::<GameEntityType>();
        specs_world.register::<BlockTag>();

        specs_world.insert(Motion::default());
        specs_world.insert(CollisionWorldWrapper::default());
        specs_world.insert(Timing::default());

        create_entity(&mut specs_world, Some(Point2::new(0.0, -280.0)), Some(GameEntityType::Board), false);
        create_entity(&mut specs_world, None, Some(GameEntityType::L), true);

        for line in 0..21 {
            create_entity(&mut specs_world, Some(Point2::new(0.0, -260.0 + f32::from_i32(line).unwrap() * 20.0)), Some(GameEntityType::Line), false);
        }

        make_input_binding();

        Ok(MainState {
            input_state: InputState::new(),
            specs_world,
            movement_system: MovementSystem,
            collisions_system: CollisionsSystem,
            control_system: ControlSystem,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.collisions_system.run_now(&self.specs_world);
            self.movement_system.run_now(&self.specs_world);
            self.control_system.run_now(&self.specs_world);

            self.specs_world.maintain();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        let position = self.specs_world.read_storage::<Position>();
        let mut image = self.specs_world.write_storage::<Image>();
        let collision_handle = self.specs_world.read_storage::<CollisionObjectHandle>();
        let collision_world = self.specs_world.fetch::<CollisionWorldWrapper>();

        for (position, image, collision_handle) in (&position, &mut image, &collision_handle).join() {
            graphics::draw(
                ctx,
                image,
                DrawParam::default()
                    .dest::<mint::Point2<f32>>(position.point.into()),
            ).unwrap_or_else(|err| println!("draw error {:?}", err));

            if let Some(handle) = collision_handle.deref() {
                if let Some(collision_object) = collision_world.collision_object(*handle) {
                    image.vectors = collision_object.data().vectors.clone();
                }
            }
        }

        // self.draw_aabbs(ctx);
        // self.draw_collision_contacts(ctx);

        graphics::present(ctx)?;

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
                KeyCode::Q => {
                    self.input_state.update_button_down(Buttons::RotateLeft)
                }
                KeyCode::W => {
                    self.input_state.update_button_down(Buttons::RotateRight)
                }
                _ => (),
            }

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
            KeyCode::Q => {
                self.input_state.update_button_up(Buttons::RotateLeft)
            }
            KeyCode::W => {
                self.input_state.update_button_up(Buttons::RotateRight)
            }
            _ => (),
        }

        let mut input_state = self.specs_world.write_resource::<Motion>();
        *input_state = Motion::new(&self.input_state);
    }
}


impl MainState {
    #[allow(dead_code)]
    fn draw_aabbs(&self, ctx: &mut Context) {
        let collision_world: Fetch<CollisionWorldWrapper> = self.specs_world.fetch();

        for collision_object in collision_world.collision_objects() {
            let aabb = collision_object.1.compute_aabb();
            let mb = &mut graphics::MeshBuilder::new();

            mb.rectangle(
                DrawMode::stroke(1.0),
                Rect::new(
                    world_to_screen_coordinates(
                        SCREEN_WIDTH,
                        SCREEN_HEIGHT,
                        mint::Point2::from([aabb.mins.x, aabb.mins.y])).x,
                    world_to_screen_coordinates(
                        SCREEN_WIDTH,
                        SCREEN_HEIGHT,
                        mint::Point2::from([aabb.maxs.x, aabb.maxs.y])).y,
                    aabb.maxs.x - aabb.mins.x,
                    aabb.maxs.y - aabb.mins.y),
                graphics::WHITE,
            );

            let mesh: Mesh = mb.build(ctx).unwrap();

            graphics::draw(
                ctx,
                &mesh,
                DrawParam::new(),
            ).unwrap();
        }
    }
    #[allow(dead_code)]
    fn draw_collision_contacts(&self, ctx: &mut Context) {
        let collisions: ReadStorage<Collision> = self.specs_world.read_storage::<Collision>();

        for collision in (&collisions, ).join() {
            let mb = &mut graphics::MeshBuilder::new();

            mb.circle(
                DrawMode::fill(),
                world_to_screen_coordinates(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    mint::Point2::from([collision.0.contact.world1.x, collision.0.contact.world1.y])),
                5.0,
                1.0,
                Color::from_rgb(255, 0, 0)
            );

            mb.circle(
                DrawMode::fill(),
                world_to_screen_coordinates(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    mint::Point2::from([collision.0.contact.world2.x, collision.0.contact.world2.y])),
                5.0,
                1.0,
                Color::from_rgb(255, 0, 0)
            );

            let mesh: Mesh = mb.build(ctx).unwrap();

            graphics::draw(
                ctx,
                &mesh,
                DrawParam::new(),
            ).unwrap();
        }
    }

    pub fn draw_score_board(
        ctx: &mut Context,
        score_board: &ScoreBoard,
        shared_state: &SharedState,
    ) -> GameResult {
        let mut lines = Text::new(format!("LINES: {}", score_board.lines));
        lines.set_font(shared_state.assets.font, Scale::uniform(10.0));

        let mut score = Text::new(format!("SCORE: {}", score_board.score));
        score.set_font(shared_state.assets.font, Scale::uniform(10.0));

        let mut level = Text::new(format!("LEVEL: {}", score_board.level));
        level.set_font(shared_state.assets.font, Scale::uniform(10.0));

        let mut next_piece = Text::new("NEXT");
        next_piece.set_font(shared_state.assets.font, Scale::uniform(10.0));

        graphics::draw(
            ctx,
            &lines,
            (
                world_to_screen_coordinates(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    mint::Point2::from([0.0, 0.0]),
                ),
                graphics::WHITE
            ),
        )?;

        graphics::draw(
            ctx,
            &score,
            (
                world_to_screen_coordinates(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    mint::Point2::from([0.0, 0.0]),
                ),
                graphics::WHITE
            ),
        )?;

        graphics::draw(
            ctx,
            &level,
            (
                world_to_screen_coordinates(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    mint::Point2::from([0.0, 0.0]),
                ),
                graphics::WHITE
            ),
        )?;

        graphics::draw(
            ctx,
            &next_piece,
            (
                world_to_screen_coordinates(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    mint::Point2::from([0.0, 0.0]),
                ),
                graphics::WHITE
            ),
        )
    }
}