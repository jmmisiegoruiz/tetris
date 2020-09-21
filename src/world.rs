use ggez::nalgebra::{MatrixMN, Matrix2, U21, U12, U20, U10};
use arrayvec::ArrayVec;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, POINTS_FOR_ONE_LINE, POINTS_FOR_TWO_LINES, POINTS_FOR_THREE_LINES, POINTS_FOR_MORE_THAN_THREE_LINES};
use std::convert::TryFrom;
use crate::types::{WorldPoint2, WorldVector2};

#[derive(Debug)]
pub enum TetriminoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetriminoType {
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
        let pos = WorldPoint2::new(5, 1);
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
        }    }
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
