use core::cmp::Ordering;

use alloc::vec::Vec;
use flipperzero::furi::time::Instant;
use flipperzero_sys::{
    random, IconRotation, IconRotation_IconRotation0, IconRotation_IconRotation180,
    IconRotation_IconRotation270, IconRotation_IconRotation90,
};

use crate::constants::{X_CELL_COUNT, Y_CELL_COUNT};

#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

/// Implement rotation for direction
impl From<&Direction> for IconRotation {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => IconRotation_IconRotation0,
            Direction::Down => IconRotation_IconRotation180,
            Direction::Left => IconRotation_IconRotation270,
            Direction::Right => IconRotation_IconRotation90,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Manager {
    pub position: Position,
    pub direction: Direction,
    pub time_of_death: Option<Instant>,
}

impl Manager {
    pub fn hunt(&mut self, target: Position, old_managers: Vec<Manager>) -> Position {
        // Dead managers stay at their position.
        if self.time_of_death.is_some() {
            return self.position;
        }

        let random_value = unsafe { random() % 100 };
        let normalized_value = random_value as f64 / 100.0;

        let x_direction = match self.position.x.cmp(&target.x) {
            Ordering::Less => Some(Direction::Right),
            Ordering::Equal => None,
            Ordering::Greater => Some(Direction::Left),
        };
        let y_direction = match self.position.y.cmp(&target.y) {
            Ordering::Less => Some(Direction::Down),
            Ordering::Equal => None,
            Ordering::Greater => Some(Direction::Up),
        };
        let direction = match (x_direction, y_direction) {
            (Some(x), Some(y)) => {
                // Randomly choose between x or y direction when both are available.
                if normalized_value < 0.5 {
                    y
                } else {
                    x
                }
            }
            (Some(x), None) => x,
            (None, Some(y)) => y,
            (None, None) => return self.position,
        };
        let new_position = self.step(direction);

        // Only move if the field is free
        if old_managers
            .iter()
            .any(|old_manager| old_manager.position.eq(&new_position))
        {
            self.position
        } else {
            self.step(direction)
        }
    }
}

impl Movement for Manager {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_direction_mut(&mut self) -> &mut Direction {
        &mut self.direction
    }
}

#[derive(Default)]
pub struct Forklift {
    pub position: Position,
    pub direction: Direction,
}

impl Movement for Forklift {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_direction_mut(&mut self) -> &mut Direction {
        &mut self.direction
    }
}

pub struct GameState {
    pub forklift: Forklift,
    pub managers: Vec<Manager>,
}

pub trait Movement {
    fn get_position(&self) -> Position;

    fn get_direction_mut(&mut self) -> &mut Direction;

    fn step(&mut self, direction: Direction) -> Position {
        let mut new_position = self.get_position();
        match direction {
            Direction::Up => {
                if self.get_position().y > 0 {
                    new_position.y -= 1;
                }
            }
            Direction::Down => {
                if self.get_position().y < Y_CELL_COUNT - 1 {
                    new_position.y += 1;
                }
            }
            Direction::Left => {
                if self.get_position().x > 0 {
                    new_position.x -= 1;
                }
            }
            Direction::Right => {
                if self.get_position().x < X_CELL_COUNT - 1 {
                    new_position.x += 1;
                }
            }
        }
        *self.get_direction_mut() = direction;
        new_position
    }
}
