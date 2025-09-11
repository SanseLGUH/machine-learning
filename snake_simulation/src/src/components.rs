use bevy::{prelude::*, asset::uuid::Uuid};
use smart_default::SmartDefault;
use rand::prelude::*;

use crate::{settings::AiMethods, plugins::snake::ai::*};

#[derive(Default)]
pub struct Tail {
	pub entitys: Vec<Position>,
	pub last_position: Option< Position >
}

#[derive(SmartDefault, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Snake {
	#[default(_code = "Uuid::new_v4()")]
	id: Uuid
} 

#[derive(Component, SmartDefault)]
pub struct Head {
	#[default(_code = "AiMethods::Qlearn(Qlearning::new())")]
	pub intelligence: AiMethods,
	#[default(_code = "Timer::from_seconds( 0.3, TimerMode::Repeating)")]
    pub timer: Timer,
    pub direction: Direction,
    pub eat_count: u32
}

#[derive(Component, Debug)]
pub struct Segment {
	pub owner_index: u32,
	pub index: u32
}

impl Segment {
	pub fn with_owner( owner: u32, index: u32 ) -> Self {
		Self {
			owner_index: owner,
			index: index
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
	pub index: u32
}

impl GrowthEvent {
	pub fn new( owner_index: u32, index: u32 ) -> Self {
		Self {
			owner_index: owner_index,
			index: index
		}
	}
}

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

#[derive(Default, Debug, Component, Reflect, PartialEq, Clone, Copy)]
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