#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Still,
}

#[derive(Debug, Copy, Clone, serde::Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub type Section = u8;
