use specs::*;
use crate::ecs::resources::{Motion, Timing};
use crate::ecs::components::{Position, ControllableTag, CollisionBox, Collision, Image};
use ncollide2d::query;
use ncollide2d::na;
use std::f32::consts::{FRAC_PI_2};
use crate::world::{TetriminoType, Rotation};
use crate::constants::{GRID_STEP, DESIRED_FPS, FALL_TIME};

pub struct MovementSystem;

pub struct CollisionSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Write<'a, Motion>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, ControllableTag>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Image>,
        WriteStorage<'a, CollisionBox>,
        Write<'a, Timing>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut motion_storage,
            mut position_storage,
            controlled_storage,
            collision_storage,
            mut image_storage,
            mut collision_box_storage,
            mut timing_storage
        ) = data;

        for (pos, _, _, image, collision_box) in
        (&mut position_storage,
         &controlled_storage,
         !&collision_storage,
         &mut image_storage,
         &mut collision_box_storage,
        ).join() {

            let seconds = 1.0 / (DESIRED_FPS as f32);
            timing_storage.fall_timeout -= seconds;

            if timing_storage.fall_timeout < 0.0 {
                pos.position.y = pos.position.y - GRID_STEP;
                timing_storage.fall_timeout = FALL_TIME;
            }

            if motion_storage.up {
                pos.position.y = pos.position.y + GRID_STEP;
            }
            if motion_storage.down {
                pos.position.y = pos.position.y - GRID_STEP;
            }
            if motion_storage.left {
                pos.position.x = pos.position.x - GRID_STEP;
            }
            if motion_storage.right {
                pos.position.x = pos.position.x + GRID_STEP;
            }
            if motion_storage.rotation_right {
                image.rotation = image.rotation + FRAC_PI_2;
                collision_box.rotation = collision_box.rotation + FRAC_PI_2;
                collision_box.support_map = TetriminoType::collision_box(
                    collision_box.tetrimino_type.as_ref().unwrap(),
                    Rotation::from_value(collision_box.rotation).unwrap(),
                ).unwrap().support_map;
            }
        }
        motion_storage.reset();
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Motion>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, CollisionBox>,
        ReadStorage<'a, ControllableTag>,
        WriteStorage<'a, Collision>,
        Read<'a, Timing>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
            motion_storage,
            position_storage,
            collision_box_storage,
            controlled_storage,
            mut collision_storage,
            timing_storage
        ) = data;

        // First find the player collision boxes, we don't assume a single player
        for (player_position, player_entity, player_box, _) in (&position_storage, &entities, &collision_box_storage, &controlled_storage).join() {
            // Now check all entities with a collision box that aren't player controlled

            collision_storage.remove(player_entity);

            for (pos, coll_box, _) in (&position_storage, &collision_box_storage, !&controlled_storage).join() {
                let mut next_player_position = Position {
                    position: mint::Point2::from([player_position.position.x, player_position.position.y]),
                };

                let mut next_player_rotation = player_box.rotation;

                let seconds = 1.0 / (DESIRED_FPS as f32);
                let next_fall_timeout = timing_storage.fall_timeout - seconds;

                if next_fall_timeout < 0.0 {
                    next_player_position.position.y = next_player_position.position.y - GRID_STEP;
                }

                if motion_storage.up {
                    next_player_position.position.y = next_player_position.position.y + GRID_STEP;
                }
                if motion_storage.down {
                    next_player_position.position.y = next_player_position.position.y - GRID_STEP;
                }
                if motion_storage.left {
                    next_player_position.position.x = next_player_position.position.x - GRID_STEP;
                }
                if motion_storage.right {
                    next_player_position.position.x = next_player_position.position.x + GRID_STEP;
                }
                if motion_storage.rotation_right {
                    next_player_rotation = next_player_rotation + FRAC_PI_2;
                }

                let m1 = na::Isometry2::new(na::Vector2::new(next_player_position.position.x, next_player_position.position.y), na::zero());
                let m2 = na::Isometry2::new(na::Vector2::new(pos.position.x, pos.position.y), na::zero());

                let contact_result = query::contact(
                    &m1,
                    &TetriminoType::collision_box(
                        &player_box.tetrimino_type.as_ref().unwrap(),
                        Rotation::from_value(next_player_rotation).unwrap(),
                    ).unwrap().support_map,
                    &m2,
                    &coll_box.support_map,
                    0.0);

                if let Some(contact) = contact_result {
                    let paralyze = !motion_storage.up &&
                        !motion_storage.down &&
                        !motion_storage.left &&
                        !motion_storage.right &&
                        !motion_storage.rotation_right;

                    let collision = Collision {
                        contact,
                        paralyze
                    };
                    collision_storage.insert(player_entity, collision).unwrap();
                }
            }
        }
    }
}
