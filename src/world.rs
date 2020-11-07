use ggez::nalgebra::{MatrixMN, Matrix2, U21, U12, U20, U10};
use arrayvec::ArrayVec;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, POINTS_FOR_ONE_LINE, POINTS_FOR_TWO_LINES, POINTS_FOR_THREE_LINES, POINTS_FOR_MORE_THAN_THREE_LINES, TOP_BOUNDARY, SCALE_FACTOR, GRID_STEP};
use std::convert::TryFrom;
use crate::types::{WorldPoint2, WorldVector2};
use crate::ecs::components::{CollisionBox, Image};
use ncollide2d::na;
use ncollide2d::na::Isometry2;
use ncollide2d::shape::{ShapeHandle, Cuboid, Compound};
use ggez::{GameResult, Context};
use ggez::graphics::{Color, WHITE};
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Debug, Clone)]
pub enum TetriminoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone)]
pub enum Rotation {
    Zero,
    FracPi2,
    Pi,
    Frac3Pi2,
}

impl Rotation {
    pub fn from_value(value: f32) -> Option<Rotation> {
        let value_to_test = ((value / (2.0 * PI)).fract() * 100.0).round() / 100.0;

        if value_to_test == 0.0 || value_to_test == 1.0 {
            Some(Rotation::Zero)
        } else if value_to_test == 0.25 {
            Some(Rotation::Frac3Pi2)
        } else if value_to_test == 0.5 {
            Some(Rotation::Pi)
        } else if value_to_test == 0.75 {
            Some(Rotation::FracPi2)
        } else {
            None
        }
    }

    pub fn to_value(&self) -> f32 {
        match self {
            Rotation::Zero => 0.0,
            Rotation::FracPi2 => FRAC_PI_2,
            Rotation::Pi => PI,
            Rotation::Frac3Pi2 => 3.0 * FRAC_PI_2,
        }
    }
}

impl TetriminoType {
    pub fn collision_box(&self, rotation: Rotation) -> GameResult<CollisionBox> {
        match self {
            TetriminoType::T => {
                let mut shapes = Vec::new();
                let shape1 = ShapeHandle::new(Cuboid::new(na::Vector2::new(0.5 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));
                let shape2 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.5 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

                let delta1 = Isometry2::new(na::Vector2::new(0.0, 0.0), na::zero());
                shapes.push((delta1, shape1));

                match rotation {
                    Rotation::Zero => {
                        let delta2 = Isometry2::new(na::Vector2::new(0.0, -1.0 * GRID_STEP), na::zero());
                        shapes.push((delta2, shape2));
                    }
                    Rotation::FracPi2 => {
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, 0.0), FRAC_PI_2);
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Pi => {
                        let delta2 = Isometry2::new(na::Vector2::new(0.0, 1.0 * GRID_STEP), na::zero());
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, 0.0), FRAC_PI_2);
                        shapes.push((delta2, shape2));
                    }
                }
                Ok(CollisionBox {
                    rotation: rotation.to_value(),
                    support_map: Compound::new(shapes),
                    tetrimino_type: Some((*self).clone()),
                })
            }
            TetriminoType::I => {
                let mut shapes = Vec::new();
                let shape = ShapeHandle::new(Cuboid::new(na::Vector2::new(2.0 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

                match rotation {
                    Rotation::Zero => {
                        let delta = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, 0.0), na::zero());
                        shapes.push((delta, shape));
                    }
                    Rotation::FracPi2 => {
                        let delta = Isometry2::new(na::Vector2::new(0.0, 0.5 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta, shape));
                    }
                    Rotation::Pi => {
                        let delta = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP, 0.0), na::zero());
                        shapes.push((delta, shape));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta = Isometry2::new(na::Vector2::new(0.0, -0.5 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta, shape));
                    }
                }

                Ok(CollisionBox {
                    support_map: Compound::new(shapes),
                    rotation: Rotation::Zero.to_value(),
                    tetrimino_type: Some((*self).clone()),
                })
            }
            TetriminoType::J => {
                let mut shapes = Vec::new();
                let shape1 = ShapeHandle::new(Cuboid::new(na::Vector2::new(0.5 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));
                let shape2 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.5 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

                let delta1 = Isometry2::new(na::Vector2::new(0.0, 0.0), na::zero());
                shapes.push((delta1, shape1));

                match rotation {
                    Rotation::Zero => {
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, -1.0 * GRID_STEP), na::zero());
                        shapes.push((delta2, shape2));
                    }
                    Rotation::FracPi2 => {
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, 1.0 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Pi => {
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, 1.0 * GRID_STEP), na::zero());
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, -1.0 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta2, shape2));
                    }
                }
                Ok(CollisionBox {
                    rotation: rotation.to_value(),
                    support_map: Compound::new(shapes),
                    tetrimino_type: Some((*self).clone()),
                })
            }
            TetriminoType::L => {
                let mut shapes = Vec::new();
                let shape1 = ShapeHandle::new(Cuboid::new(na::Vector2::new(0.5 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));
                let shape2 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.5 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

                let delta1 = Isometry2::new(na::Vector2::new(0.0, 0.0), na::zero());
                shapes.push((delta1, shape1));

                match rotation {
                    Rotation::Zero => {
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, -1.0 * GRID_STEP), na::zero());
                        shapes.push((delta2, shape2));
                    }
                    Rotation::FracPi2 => {
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, -1.0 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Pi => {
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, 1.0 * GRID_STEP), na::zero());
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, 1.0 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta2, shape2));
                    }
                }
                Ok(CollisionBox {
                    rotation: rotation.to_value(),
                    support_map: Compound::new(shapes),
                    tetrimino_type: Some((*self).clone()),
                })
            }
            TetriminoType::Z => {
                let mut shapes = Vec::new();
                let shape1 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.0 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));
                let shape2 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.0 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

                match rotation {
                    Rotation::Zero => {
                        let delta1 = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP, 0.0), na::zero());
                        let delta2 = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, -1.0 * GRID_STEP), na::zero());
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                    Rotation::FracPi2 => {
                        let delta1 = Isometry2::new(na::Vector2::new(0.0, -0.5 * GRID_STEP), FRAC_PI_2);
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, 0.5 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Pi => {
                        let delta1 = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, 0.0), na::zero());
                        let delta2 = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP, 1.0 * GRID_STEP), na::zero());
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta1 = Isometry2::new(na::Vector2::new(0.0, 0.5 * GRID_STEP), FRAC_PI_2);
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, -0.5 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                }
                Ok(CollisionBox {
                    rotation: rotation.to_value(),
                    support_map: Compound::new(shapes),
                    tetrimino_type: Some((*self).clone()),
                })
            }
            TetriminoType::S => {
                let mut shapes = Vec::new();
                let shape1 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.0 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));
                let shape2 = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.0 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

                match rotation {
                    Rotation::Zero => {
                        let delta1 = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, 0.0), na::zero());
                        let delta2 = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP, -1.0 * GRID_STEP), na::zero());
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                    Rotation::FracPi2 => {
                        let delta1 = Isometry2::new(na::Vector2::new(0.0, 0.5 * GRID_STEP), FRAC_PI_2);
                        let delta2 = Isometry2::new(na::Vector2::new(1.0 * GRID_STEP, -0.5 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Pi => {
                        let delta1 = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP, 0.0), na::zero());
                        let delta2 = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, 1.0 * GRID_STEP), na::zero());
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta1 = Isometry2::new(na::Vector2::new(0.0, -0.5 * GRID_STEP), FRAC_PI_2);
                        let delta2 = Isometry2::new(na::Vector2::new(-1.0 * GRID_STEP, 0.5 * GRID_STEP), FRAC_PI_2);
                        shapes.push((delta1, shape1));
                        shapes.push((delta2, shape2));
                    }
                }
                Ok(CollisionBox {
                    rotation: rotation.to_value(),
                    support_map: Compound::new(shapes),
                    tetrimino_type: Some((*self).clone()),
                })
            }
            TetriminoType::O => {
                let mut shapes = Vec::new();
                let shape = ShapeHandle::new(Cuboid::new(na::Vector2::new(1.0 * SCALE_FACTOR, 1.0 * SCALE_FACTOR)));

                match rotation {
                    Rotation::Zero => {
                        let delta = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, -0.5 * GRID_STEP), na::zero());
                        shapes.push((delta, shape));
                    }
                    Rotation::FracPi2 => {
                        let delta = Isometry2::new(na::Vector2::new(0.5 * GRID_STEP, 0.5 * GRID_STEP), na::zero());
                        shapes.push((delta, shape));
                    }
                    Rotation::Pi => {
                        let delta = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP, 0.5 * GRID_STEP), na::zero());
                        shapes.push((delta, shape));
                    }
                    Rotation::Frac3Pi2 => {
                        let delta = Isometry2::new(na::Vector2::new(-0.5 * GRID_STEP,-0.5 * GRID_STEP), na::zero());
                        shapes.push((delta, shape));
                    }
                }

                Ok(CollisionBox {
                    rotation: rotation.to_value(),
                    support_map: Compound::new(shapes),
                    tetrimino_type: Some((*self).clone()),
                })
            }
        }
    }

    pub fn image(&self, ctx: &mut Context, rotation: f32, scale: f32) -> GameResult<Image> {
        let mut points: Vec<mint::Point2<f32>> = Vec::new();
        let mut color: Color = WHITE;
        let mut offset: mint::Point2<f32> = mint::Point2::from([0.0, 0.0]);

        match self {
            TetriminoType::I => {
                points.push(mint::Point2::from([2.0, -0.5]));
                points.push(mint::Point2::from([2.0, 0.5]));
                points.push(mint::Point2::from([-2.0, 0.5]));
                points.push(mint::Point2::from([-2.0, -0.5]));

                color = Color::from_rgb(165, 42, 42);
                offset = mint::Point2::from([-0.5, 0.0]);
            }
            TetriminoType::J => {
                points.push(mint::Point2::from([-0.5, -0.5]));
                points.push(mint::Point2::from([0.5, -0.5]));
                points.push(mint::Point2::from([0.5, 0.5]));
                points.push(mint::Point2::from([2.5, 0.5]));
                points.push(mint::Point2::from([2.5, 1.5]));
                points.push(mint::Point2::from([-0.5, 1.5]));
            }
            TetriminoType::L => {
                points.push(mint::Point2::from([-0.5, -0.5]));
                points.push(mint::Point2::from([0.5, -0.5]));
                points.push(mint::Point2::from([0.5, 1.5]));
                points.push(mint::Point2::from([-2.5, 1.5]));
                points.push(mint::Point2::from([-2.5, 0.5]));
                points.push(mint::Point2::from([-0.5, 0.5]));

                color = Color::from_rgb(255, 0, 255);
            }
            TetriminoType::O => {
                points.push(mint::Point2::from([-0.5, -0.5]));
                points.push(mint::Point2::from([0.5, -0.5]));
                points.push(mint::Point2::from([1.5, -0.5]));
                points.push(mint::Point2::from([1.5, 1.5]));
                points.push(mint::Point2::from([0.0, 1.5]));
                points.push(mint::Point2::from([-0.5, 1.5]));

                color = Color::from_rgb(0, 0, 255);
            }
            TetriminoType::S => {
                points.push(mint::Point2::from([1.5, -0.5]));
                points.push(mint::Point2::from([-0.5, -0.5]));
                points.push(mint::Point2::from([-0.5, 0.5]));
                points.push(mint::Point2::from([-1.5, 0.5]));
                points.push(mint::Point2::from([-1.5, 1.5]));
                points.push(mint::Point2::from([0.5, 1.5]));
                points.push(mint::Point2::from([0.5, 0.5]));
                points.push(mint::Point2::from([1.5, 0.5]));

                color = Color::from_rgb(0, 255, 0);
            }
            TetriminoType::T => {
                points.push(mint::Point2::from([-0.5, -0.5]));
                points.push(mint::Point2::from([0.5, -0.5]));
                points.push(mint::Point2::from([0.5, 0.5]));
                points.push(mint::Point2::from([1.5, 0.5]));
                points.push(mint::Point2::from([1.5, 1.5]));
                points.push(mint::Point2::from([-1.5, 1.5]));
                points.push(mint::Point2::from([-1.5, 0.5]));
                points.push(mint::Point2::from([-0.5, 0.5]));

                color = Color::from_rgb(255, 0, 0);
            }
            TetriminoType::Z => {
                points.push(mint::Point2::from([-1.5, -0.5]));
                points.push(mint::Point2::from([0.5, -0.5]));
                points.push(mint::Point2::from([0.5, 0.5]));
                points.push(mint::Point2::from([1.5, 0.5]));
                points.push(mint::Point2::from([1.5, 1.5]));
                points.push(mint::Point2::from([-0.5, 1.5]));
                points.push(mint::Point2::from([-0.5, 0.5]));
                points.push(mint::Point2::from([-1.5, 0.5]));

                color = Color::from_rgb(0, 255, 255);
            }
        }

        Ok(Image::new(ctx, points, color, offset, rotation, scale))
    }

    pub fn to_code(&self) -> u8 {
        match self {
            TetriminoType::I => 1,
            TetriminoType::J => 2,
            TetriminoType::L => 3,
            TetriminoType::O => 4,
            TetriminoType::S => 5,
            TetriminoType::T => 6,
            TetriminoType::Z => 7,
        }
    }

    pub fn from_code(code: u8) -> Option<TetriminoType> {
        match code {
            1 => Option::from(TetriminoType::I),
            2 => Option::from(TetriminoType::J),
            3 => Option::from(TetriminoType::L),
            4 => Option::from(TetriminoType::O),
            5 => Option::from(TetriminoType::S),
            6 => Option::from(TetriminoType::T),
            7 => Option::from(TetriminoType::Z),
            _ => Option::None,
        }
    }
}

#[derive(Debug)]
pub enum BoardType {
    EMPTY,
    BLOCK,
    LIMIT,
}

impl BoardType {
    pub fn from_code(code: u8) -> Option<BoardType> {
        match code {
            0 => Option::from(BoardType::EMPTY),
            99 => Option::from(BoardType::LIMIT),
            _ => Option::from(BoardType::BLOCK),
        }
    }
}

#[derive(Debug)]
enum Direction {
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Board {
    pub data: MatrixMN<u8, U21, U12>,
    pub height: f32,
    pub width: f32,
}

impl Board {
    pub fn collision_box(&self) -> GameResult<CollisionBox> {
        let delta = Isometry2::new(na::Vector2::new(0.0, 0.0), na::zero());

        let mut shapes = Vec::new();
        let shape = ShapeHandle::new(Cuboid::new(na::Vector2::new(20.0 * SCALE_FACTOR, 0.5 * SCALE_FACTOR)));

        shapes.push((delta, shape));

        Ok(CollisionBox {
            support_map: Compound::new(shapes),
            rotation: Rotation::Zero.to_value(),
            tetrimino_type: None,
        })
    }

    pub fn image(&self, ctx: &mut Context, rotation: f32, scale: f32) -> GameResult<Image> {
        let mut points: Vec<mint::Point2<f32>> = Vec::new();
        let offset: mint::Point2<f32> = mint::Point2::from([0.0, 0.0]);

        points.push(mint::Point2::from([20.0, -0.5]));
        points.push(mint::Point2::from([20.0, 0.5]));
        points.push(mint::Point2::from([-20.0, 0.5]));
        points.push(mint::Point2::from([-20.0, -0.5]));

        Ok(Image::new(ctx, points, WHITE, offset, rotation, scale))
    }

    pub fn new() -> Board {
        let data: MatrixMN<u8, U21, U12> = MatrixMN::<u8, U20, U10>::zeros()
            .insert_row(20, 99)
            .insert_column(0, 99)
            .insert_column(11, 99);
        Board {
            data,
            height: BOARD_HEIGHT,
            width: BOARD_WIDTH,
        }
    }

    pub fn update(&mut self, tetrimino: &Tetrimino, score: &mut ScoreBoard) -> u8 {
        if let Some(matrix_index) = to_matrix_index(tetrimino.pos.x, tetrimino.pos.y) {
            *self.data.index_mut(matrix_index) = tetrimino.kind.to_code();
        }
        for vector in tetrimino.vectors.iter() {
            if let Some(matrix_index) = to_matrix_index(tetrimino.pos.x + vector.x, tetrimino.pos.y + vector.y) {
                *self.data.index_mut(matrix_index) = tetrimino.kind.to_code();
            }
        }

        let mut updated_data: MatrixMN<u8, U21, U12> = MatrixMN::<u8, U20, U10>::zeros()
            .insert_row(20, 99)
            .insert_column(0, 99)
            .insert_column(11, 99);
        let mut updated_data_row_index = 19;
        let mut cleaned_lines: u8 = 0;

        for data_row_index in (0..=19).rev() {
            let mut row = self.data.row_mut(data_row_index);
            let row_complete = row.column_iter().all(|element| *element.get((0, 0)).unwrap() > 0);
            if !row_complete {
                for (c, element) in row.column_iter_mut().enumerate() {
                    *updated_data.index_mut((updated_data_row_index, c)) = *element.get((0, 0)).unwrap();
                }
                if updated_data_row_index > 0 {
                    updated_data_row_index -= 1;
                }
            } else {
                score.lines += 1;
                cleaned_lines += 1;
            }
        }

        score.score = match cleaned_lines {
            0 => score.score,
            1 => score.score + POINTS_FOR_ONE_LINE,
            2 => score.score + POINTS_FOR_TWO_LINES,
            3 => score.score + POINTS_FOR_THREE_LINES,
            _ => score.score + POINTS_FOR_MORE_THAN_THREE_LINES,
        };

        score.level = match score.lines {
            0..=5 => 0,
            6..=10 => 1,
            11..=15 => 2,
            16..=20 => 3,
            21..=25 => 4,
            26..=30 => 5,
            31..=35 => 6,
            36..=40 => 7,
            41..=45 => 8,
            46..=50 => 9,
            _ => 9,
        };

        self.data = updated_data;
        cleaned_lines
    }
}

#[derive(Debug)]
pub struct Tetrimino {
    pub kind: TetriminoType,
    pub pos: WorldPoint2,
    pub vectors: [WorldVector2; 3],
}

impl Tetrimino {
    const CLOCKWISE_MATRIX: [i8; 4] = [0, -1, 1, 0];
    const COUNTER_CLOCKWISE_MATRIX: [i8; 4] = [0, 1, -1, 0];

    pub fn from(kind: &TetriminoType) -> Tetrimino {
        let pos = WorldPoint2::new(5, TOP_BOUNDARY);
        match kind {
            TetriminoType::I => Tetrimino {
                kind: TetriminoType::I,
                pos,
                vectors: [WorldVector2::new(-1, 0), WorldVector2::new(1, 0), WorldVector2::new(2, 0)],
            },
            TetriminoType::J => Tetrimino {
                kind: TetriminoType::J,
                pos,
                vectors: [WorldVector2::new(-2, 0), WorldVector2::new(-1, 0), WorldVector2::new(0, 1)],
            },
            TetriminoType::L => Tetrimino {
                kind: TetriminoType::L,
                pos,
                vectors: [WorldVector2::new(0, 1), WorldVector2::new(1, 0), WorldVector2::new(2, 0)],
            },
            TetriminoType::O => Tetrimino {
                kind: TetriminoType::O,
                pos,
                vectors: [WorldVector2::new(1, 0), WorldVector2::new(0, 1), WorldVector2::new(1, 1)],
            },
            TetriminoType::S => Tetrimino {
                kind: TetriminoType::S,
                pos,
                vectors: [WorldVector2::new(-1, 1), WorldVector2::new(0, 1), WorldVector2::new(1, 0)],
            },
            TetriminoType::T => Tetrimino {
                kind: TetriminoType::T,
                pos,
                vectors: [WorldVector2::new(-1, 0), WorldVector2::new(0, 1), WorldVector2::new(1, 0)],
            },
            TetriminoType::Z => Tetrimino {
                kind: TetriminoType::Z,
                pos,
                vectors: [WorldVector2::new(-1, -1), WorldVector2::new(0, -1), WorldVector2::new(1, 0)],
            },
        }
    }

    pub fn move_left(&mut self, board: &Board) {
        if self.can_move(Direction::LEFT, board) {
            self.pos.x += -1;
        }
    }

    pub fn move_right(&mut self, board: &Board) {
        if self.can_move(Direction::RIGHT, board) {
            self.pos.x += 1;
        }
    }

    pub fn move_down(&mut self, board: &Board) -> bool {
        let can_move = self.can_move(Direction::DOWN, board);
        if can_move {
            self.pos.y += 1;
        }
        can_move
    }

    fn can_move(&mut self, direction: Direction, board: &Board) -> bool {
        let mut can_move = true;
        for vector in self.vectors.iter() {
            match direction {
                Direction::DOWN => {
                    if !board_position_empty(self.pos.x + vector.x, self.pos.y + vector.y + 1, board) {
                        can_move = false;
                    }
                }
                Direction::LEFT => {
                    if !board_position_empty(self.pos.x + vector.x - 1, self.pos.y + vector.y, board) {
                        can_move = false;
                    }
                }
                Direction::RIGHT => {
                    if !board_position_empty(self.pos.x + vector.x + 1, self.pos.y + vector.y, board) {
                        can_move = false;
                    }
                }
            }
            if !can_move {
                break;
            }
        }
        can_move
    }

    pub fn rotate_counter_clockwise(&mut self, board: &Board) {
        let rotation_matrix = Matrix2::from_row_slice(&Tetrimino::COUNTER_CLOCKWISE_MATRIX);
        let mut new_vectors = ArrayVec::<[WorldVector2; 3]>::new();
        let mut can_rotate = true;

        for vector in self.vectors.iter() {
            let result: WorldVector2 = rotation_matrix * vector;
            if !board_position_empty(self.pos.x + result.x, self.pos.y + result.y, board) {
                can_rotate = false;
                break;
            }
            new_vectors.push(result);
        }

        if can_rotate {
            self.vectors = new_vectors.into_inner().unwrap();
        }
    }

    pub fn rotate_clockwise(&mut self, board: &Board) {
        let rotation_matrix = Matrix2::from_row_slice(&Tetrimino::CLOCKWISE_MATRIX);
        let mut new_vectors = ArrayVec::<[WorldVector2; 3]>::new();
        let mut can_rotate = true;

        for vector in self.vectors.iter() {
            let result = rotation_matrix * vector;
            if !board_position_empty(self.pos.x + result.x, self.pos.y + result.y, board) {
                can_rotate = false;
                break;
            }
            new_vectors.push(result);
        }

        if can_rotate {
            self.vectors = new_vectors.into_inner().unwrap();
        }
    }
}

pub struct ScoreBoard {
    pub level: u8,
    pub lines: u8,
    pub score: u32,
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        ScoreBoard {
            level: 0,
            lines: 0,
            score: 0,
        }
    }
}

fn to_matrix_index(x_coordinate: i8, y_coordinate: i8) -> Option<(usize, usize)> {
    match usize::try_from(x_coordinate) {
        Ok(x_index) => {
            match usize::try_from(y_coordinate) {
                Ok(y_index) => {
                    Some((y_index, x_index))
                }
                _ => {
                    None
                }
            }
        }
        _ => {
            None
        }
    }
}

fn board_position_empty(x: i8, y: i8, board: &Board) -> bool {
    let mut position_empty = true;
    if let Some(matrix_index) = to_matrix_index(x, y) {
        if let Some(value) = board.data.get(matrix_index) {
            if value > &0 {
                position_empty = false;
            }
        }
    }
    position_empty
}
