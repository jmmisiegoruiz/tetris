use ggez_goodies::input::{InputBinding};
use ggez::event::KeyCode;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Buttons {
    RotateRight,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Axes {
    Horizontal,
    Vertical,
}

pub fn make_input_binding() -> InputBinding<Axes, Buttons> {
    let ib = InputBinding::<Axes, Buttons>::new()
        .bind_key_to_button(KeyCode::Q, Buttons::RotateRight)
        .bind_key_to_axis(KeyCode::Up, Axes::Vertical, true)
        .bind_key_to_axis(KeyCode::Down, Axes::Vertical, false)
        .bind_key_to_axis(KeyCode::Left, Axes::Horizontal, false)
        .bind_key_to_axis(KeyCode::Right, Axes::Horizontal, true);
    ib
}
