use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Settings {
	arena_width: u8, 
	arena_height: u8,

	food_at_once: u8,

	world_speed: f64,

	pause: bool,
}

#[derive(Component)]
pub struct Size {
	width: u8,
	height: u8
}

impl Size {
	fn squaire(x: u8) -> Self {
		Self {
			width: x,
			height: x
		}
	}
}

#[derive(Component)]
pub struct Position {
	x: u8,
	y: u8
}

pub enum Direction {
	Left, Up, Right, Down
}