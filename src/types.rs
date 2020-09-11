use ggez::nalgebra as na;
use ggez::event::KeyCode;
use crate::SharedState;

pub type ScreenPoint2 = na::Point2<f32>;
pub type WorldPoint2 = na::Point2<i8>;
pub type WorldVector2 = na::Vector2<i8>;

pub type SceneStack = ggez_goodies::scene::SceneStack<SharedState, KeyCode>;
