use bevy::prelude::*;

use pathfinding::directed::astar;
use smart_default::SmartDefault;

use crate::{
	components::*, settings::*
};

#[derive(Debug, Reflect, SmartDefault, PartialEq)]
pub struct ModifiedAstar {
	// #[default(_code = "Position::new(  )")]
	target: Position
}

impl ModifiedAstar {
	fn run( &mut self ) -> Direction {
		Direction::Down
	}

	fn set_target( &mut self, target: Position ) {
	}	
}
