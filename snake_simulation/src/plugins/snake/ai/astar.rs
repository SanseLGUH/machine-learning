use bevy::prelude::*;

use pathfinding::directed::astar;
use smart_default::SmartDefault;

use crate::{
	components::*, settings::*
};

#[derive(Debug, Reflect, SmartDefault, PartialEq)]
pub struct Astar {
	// params: [ Position; 2 ],
	// obstacles: Vec<Position>
}

impl Astar {
	fn run( &mut self ) {
	}

	fn set_target( &mut self, target: Position ) {
	}
}