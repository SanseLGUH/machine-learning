use bevy::{prelude::*, asset::uuid::Uuid};
use rand::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeSegment {
	count: i64,
	followed_by: Entity 
}

impl SnakeSegment {
	fn followed_by( en: Entity, count: i64 ) -> Self {
		Self {
			count: count,
			followed_by: id
		}	
	}
}

#[derive(Component)]
pub struct Eattable;

#[derive(Component, Default)]
pub enum Direction {
    Right,
    Left, 

    #[default]
    Up, 
    Down
}

#[derive(Component)]
pub struct Size {
	pub width: f64,
	pub height: f64
}

impl Size {
	pub fn square(x: f64) -> Self {
		Self {
			width: x,
			height: x
		}
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