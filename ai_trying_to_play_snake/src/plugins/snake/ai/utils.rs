use crate::components::*;
// use crate::GameState;

pub fn modified_astar(current_pos: &Position, expected_pos: &Position, obstacles: Vec<Position> ) -> Position {
	todo!()
}

pub fn astar() -> Position {
	todo!()
}

pub struct ArtificialIntelligenceQlearning {
	weights: [ f32; 3 ],
}

impl Default for ArtificialIntelligenceQlearning {
	fn default() -> Self {
		Self {
			weights: [ 1., 0., 0. ]
		}
	}
}

impl ArtificialIntelligenceQlearning {
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