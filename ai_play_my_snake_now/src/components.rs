use bevy::{prelude::*, asset::uuid::Uuid};
use rand::prelude::*;

#[derive(Default)]
pub struct Tail {
	pub entitys: Vec<Position>,
	pub last_position: Option< Position >
}

#[derive(Component, Default)]
pub struct Head {
	pub tail: Tail
}

#[derive(Component)]
pub struct Segment {
	pub owner_index: u32
}

impl Segment {
	pub fn with_owner( index: u32 ) -> Self {
		Self {
			owner_index: index
		}
	}
}

#[derive(Component)]
pub struct Eattable;

#[derive(Component)]
pub struct Obstacle;

#[derive(Event)]
pub struct GrowthEvent {
	pub owner_index: u32,
	pub owner_position: Position
}

impl GrowthEvent {
	pub fn new( owner_index: u32, owner_position: Position ) -> Self {
		Self {
			owner_index: owner_index,
			owner_position: owner_position
		}
	}
}

#[derive(Reflect, Component, Default)]
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

#[derive(Default, Debug, Component, Reflect, PartialEq, Clone)]
pub struct Position {
	pub x: i32,
	pub y: i32
}

impl Position {
	pub fn new( x: i32, y: i32 ) -> Self {
		Self {
			x: x,
			y: y
		}
	}

	pub fn random(arena_w: i32, arena_h: i32) -> Self {
		let mut rng = rand::rng();

		Self {
			x: rng.random_range( 0..=arena_w ),
			y: rng.random_range( 0..=arena_h )
		}
	}
}