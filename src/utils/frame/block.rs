pub use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
    Rng,
};
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Block {
    shape: Shape,
    pub dir: Direction,
    pub pos: (i16, i16),
}

impl Block {
    pub fn new(shape: Shape, dir: Direction) -> Block {
        Block {
            shape,
            dir,
            pos: (0, 4),
        }
    }

    pub fn rotate(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
        }
    }

    pub fn draw(&self) -> [u16; 4] {
        match self.shape {
            Shape::T => [
                [0b0000, 0b0010, 0b0111, 0b0000],
                [0b0000, 0b0010, 0b0011, 0b0010],
                [0b0000, 0b0111, 0b0010, 0b0000],
                [0b0000, 0b0010, 0b0110, 0b0010],
            ][self.dir as usize],
            Shape::L => [
                [0b0000, 0b0110, 0b0010, 0b0010],
                [0b0000, 0b0001, 0b0111, 0b0000],
                [0b0000, 0b0010, 0b0010, 0b0011],
                [0b0000, 0b0111, 0b0100, 0b0000],
            ][self.dir as usize],
            Shape::J => [
                [0b0000, 0b0011, 0b0010, 0b0010],
                [0b0000, 0b0000, 0b0111, 0b0001],
                [0b0000, 0b0010, 0b0010, 0b0110],
                [0b0000, 0b0100, 0b0111, 0b0000],
            ][self.dir as usize],
            Shape::Z => [
                [0b0000, 0b0011, 0b0110, 0b0000],
                [0b0000, 0b0010, 0b0011, 0b0001],
                [0b0000, 0b0000, 0b0011, 0b0110],
                [0b0000, 0b0100, 0b0110, 0b0010],
            ][self.dir as usize],
            Shape::S => [
                [0b0000, 0b0110, 0b0011, 0b0000],
                [0b0000, 0b0001, 0b0011, 0b0010],
                [0b0000, 0b0000, 0b0110, 0b0011],
                [0b0000, 0b0010, 0b0110, 0b0100],
            ][self.dir as usize],
            Shape::O => [0b0110, 0b0110, 0b0000, 0b0000],
            Shape::I => [
                [0b0000, 0b0000, 0b1111, 0b0000],
                [0b0010, 0b0010, 0b0010, 0b0010],
                [0b0000, 0b0000, 0b1111, 0b0000],
                [0b0010, 0b0010, 0b0010, 0b0010],
            ][self.dir as usize],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Shape {
    T,
    L,
    J,
    Z,
    S,
    O,
    I,
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..7) {
            0 => Shape::T,
            1 => Shape::L,
            2 => Shape::J,
            3 => Shape::Z,
            4 => Shape::S,
            5 => Shape::O,
            6 => Shape::I,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        }
    }
}
