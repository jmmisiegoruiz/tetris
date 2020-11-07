use ggez_goodies::input::InputState;
use crate::inputs::{Axes, Buttons};

#[derive(Clone, Copy, Default)]
pub struct Motion {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub rotation_right: bool
}

impl Motion {
    pub fn new(input_state: &InputState<Axes, Buttons>) -> Self {
        Motion {
            up: input_state.get_axis_raw(Axes::Vertical) > 0.0,
            down: input_state.get_axis_raw(Axes::Vertical) < 0.0,
            left: input_state.get_axis_raw(Axes::Horizontal) < 0.0,
            right: input_state.get_axis_raw(Axes::Horizontal) > 0.0,
            rotation_right: input_state.get_button_pressed(Buttons::RotateRight),
        }
    }

    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
        self.rotation_right = false;
    }
}

#[derive(Clone, Copy)]
pub struct Timing {
    pub fall_timeout: f32,
}

impl Default for Timing {
    fn default() -> Self {
        Timing {
            fall_timeout: 1.0,
        }
    }
}
