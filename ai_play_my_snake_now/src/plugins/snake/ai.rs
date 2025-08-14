use bevy::prelude::*;
use smart_default::SmartDefault;

use crate::components::*;
use crate::settings::*;

// pub fn modified_astar(current_pos: &Position, expected_pos: &Position, obstacles: Vec<Position>, wrh_strategy: [ u8; 510 ] ) -> Position {
// 	todo!()
// }

// pub fn astar( current_pos: &Position, expected_pos: &Position, obstacles: Vec<Position> ) -> Position {
// 	todo!()
// }

#[derive(Debug, Reflect, SmartDefault, PartialEq)]
pub struct ModifiedAstar {
	// #[default(_code = "Position::new(  )")]
	target: Position
}

impl ModifiedAstar {
	fn run( &mut self ) {

	}

	fn set_target( &mut self, target: Position ) {
	}
}

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

#[derive(Debug, Reflect, SmartDefault, PartialEq)]
pub struct Qlearning {
	#[default(_code = "[ 1., 0., 0.]")]
	weights: [ f64; 3 ]
}

impl Qlearning {
	fn run(&mut self) {
	} 

	fn relu(&self) -> f64 {
		0.
	}
	fn adam(&self) -> f64 {
		0.
	}
	fn mse(&self) -> f64 {
		0.
	}
}

// weights: food path, current path, wall paths

pub fn controller( mut game_state: ResMut<GameState>, snake_heads: Query< (&Position, &mut Direction), With<Head>> )  {
	let mut change_direction: Option< Direction > = None;

	match &mut game_state.ai_method {
		AiMethods::Astar( params ) => {
		}
		AiMethods::ModifiedAstar( params ) => {
		}
		AiMethods::Qlearn( params ) => {
			params.run();
		}
	}

	// for ( pos, mut direction ) in snake_heads {
	// 	*direction = Direction::Left;
	// }
}