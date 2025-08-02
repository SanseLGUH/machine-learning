use bevy::prelude::*;
use smart_default::SmartDefault;

use crate::components::*;
use crate::settings::*;

pub fn modified_astar(current_pos: &Position, expected_pos: &Position, obstacles: Vec<Position>, wrh_strategy: [ u8; 510 ] ) -> Position {
	todo!()
}

pub fn astar( current_pos: &Position, expected_pos: &Position, obstacles: Vec<Position> ) -> Position {
	todo!()
}

#[derive(SmartDefault, Debug)]
pub struct Qlearning {
    #[default(_code = "[ 1., 0., 0. ]")]
    weights: [f32; 3],
}
	

impl Qlearning {
	fn relu() -> f64 {
		todo!()
	}

	fn adam() -> f64 {
		todo!()
	}

	fn mse() -> f64 {
		todo!()
	}	
}

// weights: food path, current path, wall paths

pub fn controller( mut game_state: ResMut<GameState>, snake_heads: Query< (&Position, &mut Direction), With<Head>> )  {
	let mut change_direction: Option< Direction > = None;

	match &mut game_state.ai_method {
		AiMethods::Astar { params } => {

		}
		AiMethods::ModifiedAstar { params, wrh_strategy } => {

		}
		AiMethods::Qlearning { weights } => {
			println!("{:?}", Qlearning::default());
		}
	}

	for ( pos, mut direction ) in snake_heads {
		*direction = Direction::Left;
	}
}