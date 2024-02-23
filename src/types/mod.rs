use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

impl Position {
    pub fn new(x : i32, y : i32) -> Position {
        Position{x,y}
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position{x : self.x + rhs.x, y : self.y + rhs.y}
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position{x : self.x - rhs.x, y : self.y - rhs.y}
    }
}
