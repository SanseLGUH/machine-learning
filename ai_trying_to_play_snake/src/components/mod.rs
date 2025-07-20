use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct Size {
	pub width: u8,
	pub height: u8
}

impl Size {
	fn squaire(x: u8) -> Self {
		Self {
			width: x,
			height: x
		}
	}
}

pub enum Direction {
	Left, Up, Right, Down
}

#[derive(Component)]
pub struct Position {
	pub x: u8,
	pub y: u8
}

impl Position {
	pub fn random() -> Self {
		let mut rng = rand::rng();

		Self {
			x: rng.random::<u8>(),
			y: rng.random::<u8>(),
		}
	}
}

