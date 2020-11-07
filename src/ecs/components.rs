use specs::{Component, VecStorage, NullStorage};
use specs_derive::*;
use ggez::{graphics, Context};
use std::sync::Arc;
use ncollide2d::shape::{Compound};

use ncollide2d::query::Contact;
use ggez::graphics::{Color, Mesh, MeshBuilder, DrawMode, WHITE};
use mint;
use mint::Point2;
use crate::world::{TetriminoType};

#[derive(Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub position: mint::Point2<f32>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Image {
    pub mesh: Arc<graphics::Mesh>,
    pub rotation: f32,
    pub offset: Point2<f32>,
    pub scale: mint::Vector2<f32>,
    pub color: Color,
}

impl Image {
    pub fn new(ctx: &mut Context, points: Vec<mint::Point2<f32>>, color: Color, offset: mint::Point2<f32>, rotation: f32, scale: f32) -> Image {
        let mesh: Mesh = MeshBuilder::new()
            .polygon(
                DrawMode::fill(),
                points.as_slice(),
                WHITE).unwrap()
            .build(ctx).unwrap();

        Image {
            mesh: Arc::new(mesh),
            rotation,
            offset,
            scale: mint::Vector2::from([scale, scale]),
            color,
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ControllableTag;

#[derive(Component)]
#[storage(VecStorage)]
pub struct CollisionBox {
    pub rotation: f32,
    pub support_map: Compound<f32>,
    pub tetrimino_type: Option<TetriminoType>,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Collision {
    pub contact: Contact<f32>,
    pub paralyze: bool,
}
