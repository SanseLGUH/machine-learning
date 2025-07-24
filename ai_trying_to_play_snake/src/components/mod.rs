use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Eattable;

#[derive(Component)]
pub struct Size {
	pub width: f64,
	pub height: f64
}

impl Size {
	pub fn squaire(x: f64) -> Self {
		Self {
			width: x,
			height: x
		}
	}
}

#[derive(Reflect)]
pub enum Direction {
	Left, Up, Right, Down
}

impl Default for Direction {
	fn default() -> Self {
		Self::Up
	}
}

#[derive(Component)]
pub struct Position {
	pub x: u8,
	pub y: u8
}

impl Position {
	pub fn random(arena_w: u8, arena_h: u8) -> Self {
		let mut rng = rand::rng();

		Self {
			x: rng.random_range( 0..=arena_w ),
			y: rng.random_range( 0..=arena_h )
		}
	}
}