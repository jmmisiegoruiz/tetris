use specs_derive::*;
use specs::{Component, VecStorage, NullStorage};
use ncollide2d::na::{Point2, Vector2, Rotation2};
use ncollide2d::pipeline::CollisionObjectSlabHandle;
use ncollide2d::query::Contact;
use rand::Rng;
use ggez::graphics::{Color, WHITE};
use ncollide2d::shape::ShapeHandle;
use crate::collisions::create_shape;
use std::ops::{Deref, DerefMut};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point2<f32>,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            point: Point2::from([0.0, 120.0]),
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Position::default()
    }

    pub fn with_position(mut self, position: Point2<f32>) -> Self {
        self.point = position;
        self
    }
}

impl From<(Point2<f32>,)> for Position
{
    fn from(position: (Point2<f32>,)) -> Self {
        Position::new()
            .with_position(position.0)
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Image {
    pub vectors: Vec<Vector2<f32>>,
    pub kind: GameEntityType,
}

impl Image {
    pub fn new() -> Self {
        Image::default()
    }

    pub fn with_kind(mut self, kind: &GameEntityType) -> Self {
        self.kind = kind.clone();
        self
    }

    pub fn with_vectors(mut self, vectors: Vec<Vector2<f32>>) -> Self {
        self.vectors = vectors;
        self
    }

    pub fn rotate(&mut self, angle: f32) {
        self.vectors = rotate_vectors(&self.vectors, angle);
    }
}

impl Default for Image {
    fn default() -> Self {
        Image {
            kind: GameEntityType::T,
            vectors: Vec::new(),
        }
    }
}

impl From<(&GameEntityType,)> for Image
{
    fn from(kind: (&GameEntityType,)) -> Self {
        Image::new()
            .with_kind(kind.0)
            .with_vectors(kind.0.vectors())
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CollisionObjectHandle(Option<CollisionObjectSlabHandle>);


impl Default for CollisionObjectHandle {
    fn default() -> Self {
        CollisionObjectHandle {
            0: None,
        }
    }
}

impl CollisionObjectHandle {
    pub fn new() -> Self {
        CollisionObjectHandle::default()
    }
}

impl Deref for CollisionObjectHandle {
    type Target = Option<CollisionObjectSlabHandle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CollisionObjectHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ControllableTag;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct StaticTag;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct LineTag;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BlockTag;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collision {
    pub contact: Contact<f32>,
    pub remove_control: bool,
}

#[derive(Debug, Clone, PartialEq, Component, Copy)]
#[storage(VecStorage)]
pub enum GameEntityType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
    Board,
    Line,
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock,
}

impl GameEntityType {
    pub fn from_code(code: u8) -> Option<GameEntityType> {
        match code {
            1 => Option::from(GameEntityType::I),
            2 => Option::from(GameEntityType::J),
            3 => Option::from(GameEntityType::L),
            4 => Option::from(GameEntityType::O),
            5 => Option::from(GameEntityType::S),
            6 => Option::from(GameEntityType::T),
            7 => Option::from(GameEntityType::Z),
            8 => Option::from(GameEntityType::Board),
            _ => Option::None,
        }
    }

    pub fn get_random() -> Option<GameEntityType> {
        let mut rng = rand::thread_rng();
        GameEntityType::from_code(rng.gen_range(1, 8))
    }

    pub fn vectors(&self) -> Vec<Vector2<f32>> {
        match &self {
            GameEntityType::T => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(-1.0, -1.0),
                    Vector2::new(0.0, -1.0),
                    Vector2::new(1.0, -1.0)
                )
            }
            GameEntityType::I => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(-1.0, 0.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(2.0, 0.0)
                )
            }
            GameEntityType::J => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(0.0, -1.0),
                    Vector2::new(-1.0, 0.0),
                    Vector2::new(-2.0, 0.0)
                )
            }
            GameEntityType::L => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(0.0, -1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(2.0, 0.0)
                )
            }
            GameEntityType::Z => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(0.0, 1.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(-1.0, 1.0)
                )
            }
            GameEntityType::S => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(0.0, 1.0),
                    Vector2::new(-1.0, 0.0),
                    Vector2::new(1.0, 1.0)
                )
            }
            GameEntityType::O => {
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(1.0, 0.0),
                    Vector2::new(0.0, -1.0),
                    Vector2::new(1.0, -1.0)
                )
            }
            GameEntityType::Board => {
                let mut vectors: Vec<Vector2<f32>> = Vec::new();

                for x in -5..7 {
                    vectors.push(Vector2::new(x as f32, 0.0))
                }

                for y in 0..22 {
                    vectors.push(Vector2::new(-5.0, y as f32))
                }

                for y in 0..22 {
                    vectors.push(Vector2::new(6.0, y as f32))
                }

                vectors
            }
            GameEntityType::Line => {
                let mut vectors: Vec<Vector2<f32>> = Vec::new();

                for x in -4..6 {
                    vectors.push(Vector2::new(x as f32, 0.0))
                }
                vectors
            }
            _ => {
                vec!(
                    Vector2::new(0.0, 0.0)
                )
            }
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            GameEntityType::I | GameEntityType::IBlock=> Color::from_rgb(165, 42, 42),
            GameEntityType::J | GameEntityType::JBlock=> WHITE,
            GameEntityType::L | GameEntityType::LBlock=> Color::from_rgb(255, 0, 255),
            GameEntityType::O | GameEntityType::OBlock=> Color::from_rgb(0, 0, 255),
            GameEntityType::S | GameEntityType::SBlock=> Color::from_rgb(0, 255, 0),
            GameEntityType::T | GameEntityType::TBlock=> Color::from_rgb(255, 0, 0),
            GameEntityType::Z | GameEntityType::ZBlock=> Color::from_rgb(0, 255, 255),
            GameEntityType::Board => Color::from_rgb(0, 255, 255),
            GameEntityType::Line => Color::from_rgb(255, 255, 0),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            GameEntityType::Board => "board",
            GameEntityType::Line => "line",
            _ => "tetrimino",
        }
    }

    pub fn get_collision_group(&self) -> usize {
        match self {
            GameEntityType::Line => 3,
            GameEntityType::Board => 2,
            _ => 1,
        }
    }

    pub fn shape(&self) -> ShapeHandle<f32> {
        create_shape(&self.vectors())
    }

    pub fn get_block_type(&self) -> Option<Self> {
        match self {
            GameEntityType::I => {Some(GameEntityType::IBlock)}
            GameEntityType::J => {Some(GameEntityType::JBlock)}
            GameEntityType::L => {Some(GameEntityType::LBlock)}
            GameEntityType::O => {Some(GameEntityType::OBlock)}
            GameEntityType::S => {Some(GameEntityType::SBlock)}
            GameEntityType::T => {Some(GameEntityType::TBlock)}
            GameEntityType::Z => {Some(GameEntityType::ZBlock)}
            _ => {None}
        }
    }
}

pub fn rotate_vectors(vectors: &Vec<Vector2<f32>>, angle: f32) -> Vec<Vector2<f32>> {
    let mut result: Vec<Vector2<f32>> = Vec::new();
    let rotation: Rotation2<f32> = Rotation2::new(angle);

    for vector in vectors {
        result.push(rotation * vector);
    }
    result
}