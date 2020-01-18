#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

pub fn get_opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::UP => Direction::DOWN,
        Direction::DOWN => Direction::UP,
        Direction::RIGHT => Direction::LEFT,
        Direction::LEFT => Direction::RIGHT,
    }
}
