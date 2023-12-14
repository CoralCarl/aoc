use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn from(tpl: (T, T)) -> Self {
        Self { x: tpl.0, y: tpl.1 }
    }
}

impl<T> Point<T> {
    pub fn distance(&self, other: Point<T>) -> usize
    where
        T: std::ops::Sub<Output = usize>,
        T: Copy + PartialOrd,
    {
        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        (x + y) as usize
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: AddAssign<T>> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: SubAssign<T>> SubAssign for Point<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .copied()
    }

    pub fn cw() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .copied()
    }

    pub fn ccw() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
        .copied()
    }

    pub fn as_point(&self) -> Point<i32> {
        match &self {
            Direction::North => Point::from((0, -1)),
            Direction::East => Point::from((1, 0)),
            Direction::South => Point::from((0, 1)),
            Direction::West => Point::from((-1, 0)),
        }
    }
}
