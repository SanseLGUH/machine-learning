use bevy::prelude::*;

use rurel::mdp::{State, Agent};

use crate::{Qtable, components::*, settings::*};
use crate::plugins::snake::ai::qlearn::{SnakeAgent, SnakeState};

pub mod qlearn;
pub mod astar;
pub mod modifiedastar;

fn head_to_state(head: &Head) -> SnakeState {
	todo!()
}

use rurel::{
    strategy::learn::QLearning, strategy::explore::RandomExploration, strategy::terminate::FixedIterations
};

pub fn controller(
	mut q_table: ResMut<Qtable>,
    game_state: Res<GameState>, 
    mut snake_heads: Query<(&Position, &mut Head)>,
    foods: Query<&Position, With<Eattable>>, 
    obstacles: Query<&Position, With<Obstacle>> 
) {
    let mut arena_obstacles: Vec<Position> = Vec::new();
    for x in 0..game_state.arena_size.width {
        arena_obstacles.push(Position { x, y: 0 });
        arena_obstacles.push(Position { x, y: game_state.arena_size.height - 1 });
    }
    for y in 0..game_state.arena_size.height {
        arena_obstacles.push(Position { x: 0, y });
        arena_obstacles.push(Position { x: game_state.arena_size.width - 1, y });
    }

    for (pos, mut head) in snake_heads.iter_mut() {
        match &head.intelligence {
            AiMethods::Qlearn => {
                let mut q_table = q_table.qt.clone();
                let mut agent = SnakeAgent::default();

                head.direction = agent.train_step(
                    q_table, &QLearning::new(0.1, 1., 1.), RandomExploration::new()
                );
            }
            AiMethods::Astar => {}, 
            AiMethods::ModifiedAstar => {}
        }
    }
}
