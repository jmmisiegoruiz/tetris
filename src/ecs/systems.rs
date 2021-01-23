use specs::{System, ReadStorage, Join, WriteStorage, Write, Read, Entities, Entity, Storage};
use crate::ecs::resources::{Motion, Timing};
use crate::ecs::components::{ControllableTag, Position, Image, CollisionObjectHandle, Collision, StaticTag, LineTag, GameEntityType, BlockTag};
use crate::constants::{GRID_STEP, DESIRED_FPS, FALL_TIME};
use ncollide2d::na::{self, Vector2, Isometry2};
use std::f32::consts::FRAC_PI_2;
use ncollide2d::query::Contact;
use crate::collisions::{CollisionObjectData, CollisionWorldWrapper, contact_query, add_to_collision_world};
use ncollide2d::pipeline::{CollisionObjectSlabHandle, CollisionObject};
use nalgebra::Point2;
use specs::world::EntitiesRes;
use specs::shred::FetchMut;
use specs::storage::MaskedStorage;
use std::ops::Deref;

pub struct MovementSystem;

pub struct CollisionsSystem;

pub struct ControlSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Write<'a, Motion>,
        Write<'a, CollisionWorldWrapper>,
        Write<'a, Timing>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Image>,
        ReadStorage<'a, CollisionObjectHandle>,
        ReadStorage<'a, ControllableTag>,
        ReadStorage<'a, StaticTag>,
        ReadStorage<'a, Collision>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut motion,
            mut collision_world,
            mut timing,
            mut position,
            mut image,
            collision_handle,
            controllable,
            static_tagged,
            collision,
        ) = data;

        let seconds = 1.0 / (DESIRED_FPS as f32);
        timing.fall_timeout -= seconds;

        for (position, handle, _, _, _) in
        (
            &mut position,
            &collision_handle,
            !&collision,
            !&static_tagged,
            &controllable,
        ).join() {
            if timing.fall_timeout < 0.0 {
                position.point = position.point - GRID_STEP * Vector2::y();
            }
            <ControlSystem>::update_position_in_collision_world(&mut collision_world, position, handle)
        }

        for (position1, image, handle, _, _) in
        (
            &mut position,
            &mut image,
            &collision_handle,
            &controllable,
            !&collision,
        ).join() {
            let mut angle = 0.0;
            if motion.up {
                position1.point = position1.point + GRID_STEP * Vector2::y();
            }

            if motion.down {
                position1.point = position1.point - GRID_STEP * Vector2::y();
            }

            if motion.left {
                position1.point = position1.point - GRID_STEP * Vector2::x();
            }

            if motion.right {
                position1.point = position1.point + GRID_STEP * Vector2::x();
            }

            if motion.rotation_right {
                angle = -FRAC_PI_2;
            }

            if motion.rotation_left {
                angle = FRAC_PI_2;
            }

            image.rotate(angle);

            if let Some(collision_object_handle) = handle.as_ref() {
                let collision_object = collision_world.get_mut(*collision_object_handle).unwrap();
                collision_object.set_position(Isometry2::new(position1.point.coords, na::zero()));
                collision_object.set_shape(collision_object.data().shape(angle));
                collision_object.data_mut().rotate(angle);
            }
        }

        collision_world.update();
        motion.reset();

        if timing.fall_timeout < 0.0 {
            timing.fall_timeout = FALL_TIME;
        }
    }
}


impl<'a> System<'a> for CollisionsSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Motion>,
        Write<'a, CollisionWorldWrapper>,
        Read<'a, Timing>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, CollisionObjectHandle>,
        WriteStorage<'a, Collision>,
        ReadStorage<'a, ControllableTag>,
        ReadStorage<'a, LineTag>,
        ReadStorage<'a, GameEntityType>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            motion,
            mut collision_world,
            timing,
            position,
            mut collision_object_handle,
            mut collision,
            controllable,
            line,
            kind
        ) = data;

        for (entity, position, kind) in
        (
            &entities,
            &position,
            &kind,
        ).join() {
            if let Some(handle) = collision_object_handle.get(entity) {
                if let None = handle.deref() {
                    add_to_collision_world(kind, entity, &mut collision_world, &mut collision_object_handle, position.point);
                }
            }
        }

        for (entity1, position1, collision_object1, _) in
        (
            &entities,
            &position,
            &collision_object_handle,
            &controllable,
        ).join() {
            collision.remove(entity1);

            for (position2, collision_object2, _, _) in
            (
                &position,
                &collision_object_handle,
                !&controllable,
                !&line,
            ).join() {
                let mut query_result: Option<Contact<f32>> = None;

                let seconds = 1.0 / (DESIRED_FPS as f32);
                let timeout = timing.fall_timeout - seconds;

                if timeout < 0.0 {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(-GRID_STEP * Vector2::y(), na::zero()),
                        &collision_world,
                    );
                }

                if motion.up && query_result == None {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(GRID_STEP * Vector2::y(), na::zero()),
                        &collision_world,
                    );
                }

                if motion.down && query_result == None {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(-GRID_STEP * Vector2::y(), na::zero()),
                        &collision_world,
                    );
                }

                if motion.left && query_result == None {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(-GRID_STEP * Vector2::x(), na::zero()),
                        &collision_world,
                    );
                }

                if motion.right && query_result == None {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(GRID_STEP * Vector2::x(), na::zero()),
                        &collision_world,
                    );
                }

                if motion.rotation_right && query_result == None {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(na::zero(), -FRAC_PI_2),
                        &collision_world,
                    );
                }

                if motion.rotation_left && query_result == None {
                    query_result = contact_query(
                        &position1,
                        &position2,
                        collision_object1,
                        collision_object2,
                        Isometry2::new(na::zero(), FRAC_PI_2),
                        &collision_world,
                    );
                }

                if let Some(contact) = query_result {
                    let col = Collision {
                        contact,
                        remove_control: !motion.any(),
                    };
                    collision.insert(entity1, col).unwrap();
                }
            }
        }
    }
}

impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, GameEntityType>,
        WriteStorage<'a, CollisionObjectHandle>,
        Write<'a, CollisionWorldWrapper>,
        WriteStorage<'a, ControllableTag>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Collision>,
        WriteStorage<'a, Image>,
        WriteStorage<'a, BlockTag>,
        ReadStorage<'a, LineTag>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut game_entity_type,
            mut collision_handle,
            mut collision_world,
            mut controllable,
            mut position,
            mut collision,
            mut image,
            mut block,
            line
        ) = data;

        let mut tetrimino_data: (Vec<Vector2<f32>>, Option<Position>, Option<GameEntityType>, bool) = (Vec::new(), None, None, true);

        for (entity, collision, kind, col_handle, pos, _) in
        (
            &entities,
            &mut collision,
            &game_entity_type,
            &mut collision_handle,
            &position,
            &controllable
        ).join() {
            if collision.remove_control {
                tetrimino_data = <ControlSystem>::get_controlled_tetrimino_data(&entities, &mut collision_world, entity, kind, col_handle, pos);
            }
        }

        <ControlSystem>::split_controlled_tetrimino_into_blocks(
            &entities,
            &mut game_entity_type,
            &mut collision_handle,
            &mut position,
            &mut image,
            &mut block,
            &mut tetrimino_data,
            &mut collision_world);

        if !tetrimino_data.3 { // Tetrimino not controlled any more
            let entity_type = GameEntityType::get_random().unwrap();
            entities.build_entity()
                .with(Position::new(), &mut position)
                .with(Image::from((&entity_type, )), &mut image)
                .with(CollisionObjectHandle::new(), &mut collision_handle)
                .with(ControllableTag, &mut controllable)
                .with(entity_type, &mut game_entity_type)
                .build();

            let mut completed_lines: Vec<u8> = Vec::new();

            for (handle, _) in
            (
                &collision_handle,
                &line,
            ).join() {
                if let Some(line_handle) = handle.as_ref() {
                    let line_col_object = collision_world.collision_object(*line_handle).unwrap();
                    let line_number: u8 = <ControlSystem>::calculate_line_number(line_col_object);
                    let block_collision_object_handles = <ControlSystem>::collect_blocks_in_contact_with_line(&mut collision_world, line_handle);
                    <ControlSystem>::update_completed_lines(&mut completed_lines, line_number, &block_collision_object_handles);
                    <ControlSystem>::remove_blocks_in_contact_with_completed_line(&entities, &mut collision_world, &block_collision_object_handles)
                }
            }

            let top_line = <ControlSystem>::calculate_top_completed_line(&mut completed_lines);

            for (pos, collision_handle, _) in (&mut position, &collision_handle, &block).join() {
                if pos.point.y > -260.0 + (top_line as f32) * 20.0 {
                    pos.point = pos.point - (completed_lines.len() as f32) * GRID_STEP * Vector2::y();
                    <ControlSystem>::update_position_in_collision_world(&mut collision_world, pos, collision_handle)
                }
            }
        }
    }
}


impl ControlSystem {
    fn update_position_in_collision_world(collision_world: &mut Write<CollisionWorldWrapper>, position: &Position, handle: &CollisionObjectHandle) {
        if let Some(collision_object_handle) = handle.as_ref() {
            let collision_object = collision_world.get_mut(*collision_object_handle).unwrap();
            collision_object.set_position(Isometry2::new(position.point.coords, na::zero()));
        }
    }

    fn calculate_top_completed_line(completed_lines: &mut Vec<u8>) -> u8 {
        let mut top_line: u8 = 0;
        completed_lines.iter()
            .for_each(|line_number| {
                if *line_number > top_line {
                    top_line = *line_number;
                }
            });
        top_line
    }

    fn get_controlled_tetrimino_data(
        entities: &Read<EntitiesRes>,
        collision_world: &mut Write<CollisionWorldWrapper>,
        entity: Entity,
        kind: &GameEntityType,
        col_handle: &mut CollisionObjectHandle,
        pos: &Position,
    ) -> (Vec<Vector2<f32>>, Option<Position>, Option<GameEntityType>, bool) {
        let mut vectors: Vec<Vector2<f32>> = Vec::new();
        if let Some(handle) = col_handle.as_ref() {
            let collision_object = collision_world.collision_object(*handle).unwrap();
            collision_object.data().vectors.iter()
                .for_each(|vector| {
                    vectors.push(Vector2::new(vector.x, vector.y));
                });
            collision_world.remove(&[*handle]);
        }

        let position = Some((*pos).clone());
        let kind = Some((*kind).clone());

        entities.delete(entity).unwrap();
        (vectors, position, kind, false)
    }

    fn split_controlled_tetrimino_into_blocks<'a>(
        entities: &Read<EntitiesRes>,
        mut game_entity_type: &mut Storage<'a, GameEntityType, FetchMut<'a, MaskedStorage<GameEntityType>>>,
        mut collision_handle: &mut Storage<'a, CollisionObjectHandle, FetchMut<'a, MaskedStorage<CollisionObjectHandle>>>,
        mut position: &mut Storage<'a, Position, FetchMut<'a, MaskedStorage<Position>>>,
        mut image: &mut Storage<'a, Image, FetchMut<'a, MaskedStorage<Image>>>,
        mut block: &mut Storage<'a, BlockTag, FetchMut<'a, MaskedStorage<BlockTag>>>,
        controlled_tetrimino_data: &mut (Vec<Vector2<f32>>, Option<Position>, Option<GameEntityType>, bool),
        collision_world: &mut Write<CollisionWorldWrapper>,
    ) {
        if let (vectors, Some(pos), Some(kind), _) = controlled_tetrimino_data {
            vectors.iter()
                .for_each(|vector| {
                    let block_type = kind.get_block_type().unwrap();
                    let position_point = Point2::new(vector.x * GRID_STEP + pos.point.x, vector.y * GRID_STEP + pos.point.y);
                    let entity = entities.build_entity()
                        .with(Position::from((position_point, )), &mut position)
                        .with(Image::from((&kind.get_block_type().unwrap(), )), &mut image)
                        .with(CollisionObjectHandle::new(), &mut collision_handle)
                        .with(block_type, &mut game_entity_type)
                        .with(BlockTag, &mut block)
                        .build();
                    add_to_collision_world(&block_type, entity, collision_world, collision_handle, position_point);
                })
        }
    }

    fn calculate_line_number(line_col_object: &CollisionObject<f32, CollisionObjectData>) -> u8 {
        ((line_col_object.position().translation.vector.y + 260.0) / 20.0) as u8
    }

    fn collect_blocks_in_contact_with_line(
        collision_world: &mut Write<CollisionWorldWrapper>,
        line_handle: &CollisionObjectSlabHandle,
    ) -> Vec<CollisionObjectSlabHandle> {
        let mut block_collision_object_handles: Vec<CollisionObjectSlabHandle> = Vec::new();
        if let Some(collision_object_handles) = collision_world.collision_objects_interacting_with(*line_handle) {
            collision_object_handles.for_each(|handle| {
                block_collision_object_handles.push(handle);
            });
        }
        block_collision_object_handles
    }

    fn remove_blocks_in_contact_with_completed_line(entities: &Read<EntitiesRes>, collision_world: &mut Write<CollisionWorldWrapper>, block_collision_object_handles: &Vec<CollisionObjectSlabHandle>) {
        if block_collision_object_handles.len() == 10 {
            block_collision_object_handles.iter().for_each(|handle| {
                if let Some(collision_object) = collision_world.collision_object(*handle) {
                    if let Some(id) = collision_object.data().entity_id {
                        let entity = entities.entity(id);
                        if entities.is_alive(entity) {
                            entities.delete(entity).unwrap();
                        }
                    }
                }
            });
            collision_world.remove(&block_collision_object_handles[..]);
        }
    }

    fn update_completed_lines(completed_lines: &mut Vec<u8>, line_number: u8, block_collision_object_handles: &Vec<CollisionObjectSlabHandle>) {
        if block_collision_object_handles.len() == 10 {
            completed_lines.push(line_number);
        }
    }
}