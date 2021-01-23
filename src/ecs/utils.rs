use ncollide2d::na::Point2;
use specs::{World, WorldExt, Builder};
use crate::ecs::components::{Position, Image, CollisionObjectHandle, ControllableTag, StaticTag, LineTag, GameEntityType};

pub fn create_entity(
    specs_world: &mut World,
    point: Option<Point2<f32>>,
    kind: Option<GameEntityType>,
    controllable: bool
) {
    let mut entity_type = GameEntityType::get_random().unwrap();
    if let Some(kind) = kind {
        entity_type = kind;
    }
    let image = Image::from((&entity_type, ));
    let mut position = Position::new();
    if let Some(point) = point {
        position = Position::from((point,));
    }

    let mut entity_builder = specs_world
        .create_entity()
        .with(position)
        .with(CollisionObjectHandle::new());

    if controllable {
        entity_builder = entity_builder.with(ControllableTag);
    }

    match entity_type {
        GameEntityType::Board => {
            entity_builder = entity_builder
                .with(StaticTag)
                .with(image);
        }
        GameEntityType::Line => {
            entity_builder = entity_builder
                .with(StaticTag)
                .with(LineTag);
        }
        _ => {
            entity_builder = entity_builder.with(image);
        }
    }

    entity_builder = entity_builder.with(entity_type);
    entity_builder.build();
}