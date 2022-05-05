pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub type Section = u8;