enum Direction {
	Up,
	Left,
	Down,
	Right,
}

impl Default for Direction {
	fn default() -> Self {
		Direction::Up
	}
}

impl Direction {
	fn rotate_left(self) -> Self {
		match self {
			Direction::Up => Direction::Left,
			Direction::Left => Direction::Down,
			Direction::Down => Direction::Right,
			Direction::Right => Direction::Up,
		}
	}

	fn rotate_right(self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}

	fn move_in_direction(position: Position, direction: Self) -> Position {
		match direction {
			Direction::Up => Position {
				x: position.x,
				y: position.y + 1,
			},
			Direction::Left => Position {
				x: position.x - 1,
				y: position.y,
			},
			Direction::Down => Position {
				x: position.x,
				y: position.y - 1,
			},
			Direction::Right => Position {
				x: position.x + 1,
				y: position.y,
			},
		}
	}
}

struct Position {
	x: i32,
	y: i32,
}

impl Default for Position {
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

struct Robot {
	position: Position,
	direction: Direction,
}

impl Default for Robot {
	fn default() -> Self {
		Self {
			position: Default::default(),
			direction: Default::default(),
		}
	}
}
