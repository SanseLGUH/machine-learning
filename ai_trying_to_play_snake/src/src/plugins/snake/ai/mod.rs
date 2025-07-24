use bevy::prelude::*;

pub mod utils;

use crate::components::*;
use crate::{GameState, AiMethods};

// enum AiMethods {
//     Astar, ModifiedAstar, QLearningAI 
// }  

pub fn controller( game_state: Res<GameState>, snake_heads: Query< &mut Position, With< SnakeHead > > )  {
	for current_pos in &snake_heads {
		match game_state.ai_method {
			AiMethods::Astar => {
				// utils::astar();
			}
			AiMethods::ModifiedAstar => {
				utils::modified_astar( current_pos, &Position { x: 1, y: 1 }, vec![ Position { x: 2, y: 3 } ] );
			}
			AiMethods::QLearningAI => {
				utils::ArtificialIntelligenceQlearning::default();
			}
		}
	}

	for mut snake_head in snake_heads {
		*snake_head = Position { x: 10, y: 10 };
	}
}