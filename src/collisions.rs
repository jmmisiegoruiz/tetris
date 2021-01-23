use ncollide2d::na::{self, Vector2, Isometry2, Point2};
use crate::constants::{COLLISION_SCALE_FACTOR, GRID_STEP};
use ncollide2d::shape::{Compound, Cuboid, ShapeHandle};
use ncollide2d::pipeline::{CollisionWorld, CollisionGroups, GeometricQueryType};
use crate::ecs::components::{rotate_vectors, GameEntityType, Position, CollisionObjectHandle};
use std::ops::{Deref, DerefMut};
use specs::world::Index;
use ncollide2d::query;
use ncollide2d::query::Contact;
use specs::{Entity, Write, Storage};
use specs::shred::FetchMut;
use specs::storage::MaskedStorage;

pub struct CollisionWorldWrapper(CollisionWorld<f32, CollisionObjectData>);

impl Default for CollisionWorldWrapper {
    fn default() -> Self {
        CollisionWorldWrapper {
            0: CollisionWorld::new(0.02)
        }
    }
}

impl Deref for CollisionWorldWrapper {
    type Target = CollisionWorld<f32, CollisionObjectData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CollisionWorldWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug)]
pub struct CollisionObjectData {
    pub name: Option<&'static str>,
    pub vectors: Vec<Vector2<f32>>,
    pub entity_id: Option<Index>,
}

impl Default for CollisionObjectData {
    fn default() -> Self {
        CollisionObjectData {
            name: None,
            vectors: Vec::new(),
            entity_id: None,
        }
    }
}

impl CollisionObjectData {
    pub fn new() -> Self {
        CollisionObjectData::default()
    }

    pub fn with_vectors(mut self, vectors: Vec<Vector2<f32>>) -> Self {
        self.vectors = vectors;
        self
    }

    pub fn with_name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_entity_id(mut self, id: Index) -> Self {
        self.entity_id = Some(id);
        self
    }

    pub fn rotate(&mut self, angle: f32) {
        self.vectors = rotate_vectors(&self.vectors, angle);
    }

    pub fn shape(&self, angle: f32) -> ShapeHandle<f32> {
        create_shape(&rotate_vectors(&self.vectors, angle))
    }
}

impl From<(&GameEntityType, Index)> for CollisionObjectData
{
    fn from((kind, id): (&GameEntityType, Index)) -> Self {
        CollisionObjectData::new()
            .with_name(kind.get_name())
            .with_vectors(kind.vectors())
            .with_entity_id(id)
    }
}

pub fn create_shape(vectors: &Vec<Vector2<f32>>) -> ShapeHandle<f32> {
    ShapeHandle::new(
        Compound::new(
            create_parts_shapes(vectors)
        )
    )
}

pub fn create_parts_shapes(vectors: &Vec<Vector2<f32>>) -> Vec<(Isometry2<f32>, ShapeHandle<f32>)> {
    vectors.iter()
        .map(|vector| {
            (
                Isometry2::new(Vector2::new(vector.x * GRID_STEP, vector.y * GRID_STEP), na::zero()),
                ShapeHandle::new(Cuboid::new(Vector2::new(0.5 * COLLISION_SCALE_FACTOR, 0.5 * COLLISION_SCALE_FACTOR)))
            )
        })
        .collect()
}


pub fn contact_query(p1: &Position,
                     p2: &Position,
                     handle1: &CollisionObjectHandle,
                     handle2: &CollisionObjectHandle,
                     isometry: Isometry2<f32>,
                     collision_world: &CollisionWorldWrapper,
) -> Option<Contact<f32>> {
    if let (Some(handle1), Some(handle2)) = (handle1.as_ref(), handle2.as_ref()) {
        if let (Some(col1), Some(col2)) = (
            collision_world.collision_object(*handle1),
            collision_world.collision_object(*handle2)
        ) {
            return query::contact(
                &(Isometry2::new(p1.point.coords, na::zero()) * isometry.translation),
                col1.data().shape(isometry.rotation.angle()).deref(),
                &Isometry2::new(p2.point.coords, na::zero()),
                col2.shape().deref(),
                0.0);
        }
    }
    None
}

pub fn add_to_collision_world<'a>(
    block_type: &GameEntityType,
    entity: Entity,
    collision_world: &mut Write<CollisionWorldWrapper>,
    collision_handle: &mut Storage<'a, CollisionObjectHandle, FetchMut<'a, MaskedStorage<CollisionObjectHandle>>>,
    position_point: Point2<f32>
) {
    let mut collision_groups = CollisionGroups::new();
    collision_groups.set_membership(&[block_type.get_collision_group()]);

    let contacts_query = GeometricQueryType::Contacts(0.0, 0.0);

    let data = CollisionObjectData::from((block_type, entity.id()));

    let new_handle = collision_world.add(
        Isometry2::new(position_point.coords, na::zero()),
        block_type.shape(),
        collision_groups,
        contacts_query,
        data).0;
    if let Some(handle) = collision_handle.get_mut(entity) {
        handle.replace(new_handle);
    }
    collision_world.update();
}
