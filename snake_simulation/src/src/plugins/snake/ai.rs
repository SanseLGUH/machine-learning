use bevy::prelude::*;
use smart_default::SmartDefault;
use rand::prelude::*;

use crate::components::*;
use crate::settings::*;


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
    #[default(_code = "[0., 1., 1.]")]
    weights: [f64; 3], // [bias, food_weight, obstacle_weight]
    #[default = 2.]
    learning_rate: f64,
}

impl Qlearning {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
			weights: [
			    rng.gen_range(-1.0..1.0),     // bias can be negative or positive
			    rng.gen_range(0.1..1.0),      // food_weight positive
			    rng.gen_range(0.1..1.0),      // obstacle_weight positive
			],
            learning_rate: rng.gen_range(0.1..1.0),
        }
    }

    /// Returns direction: 0 = UP, 1 = DOWN, 2 = RIGHT, 3 = LEFT
    pub fn run(&mut self, current_pos: Position, food: Vec<Position>, obstacles: Vec<Position>) -> f64 {
        let mut best_action = 0;
        let mut best_value = f64::MIN;

        for action in 0..4 {
            let next_pos = self.simulate_move(current_pos, action);
            let food_score = self.evaluate_food( &current_pos, &next_pos, &food);
            let obstacle_penalty = self.evaluate_obstacles(&next_pos, &obstacles);

            let q_value = self.q_value(food_score, obstacle_penalty);

            if q_value > best_value {
                best_value = q_value;
                best_action = action;
            }
        }

        best_action as f64
    }

    fn relu(&self, x: f64) -> f64 {
        x.max(0.0)
    }

    fn q_value(&self, food_score: f64, obstacle_penalty: f64) -> f64 {
        let z = self.weights[0] // bias
              + self.weights[1] * food_score
              - self.weights[2] * obstacle_penalty;

        self.relu(z)
    }

    fn simulate_move(&self, pos: Position, action: u8) -> Position {
        match action {
            0 => Position { x: pos.x, y: pos.y - 1 }, // UP
            1 => Position { x: pos.x, y: pos.y + 1 }, // DOWN
            2 => Position { x: pos.x + 1, y: pos.y }, // RIGHT
            3 => Position { x: pos.x - 1, y: pos.y }, // LEFT
            _ => pos,
        }
    }

	fn evaluate_food(&self, current: &Position, next: &Position, food: &Vec<Position>) -> f64 {
	    food.iter()
	        .map(|f| self.manhattan_distance(next, f) as f64)
	        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
	        .map(|min_dist| 1.0 / (min_dist + 0.1)) // inverse distance
	        .unwrap_or(0.0)
	}

    fn evaluate_obstacles(&self, pos: &Position, obstacles: &Vec<Position>) -> f64 {
        obstacles.iter()
            .map(|o| 1.0 / (self.manhattan_distance(pos, o) as f64 + 0.1))
            .sum()
    }

    fn manhattan_distance(&self, a: &Position, b: &Position) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }
}

// Controller system for snake AI (Q-learning, A*, etc.)
pub fn controller(
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
        match &mut head.intelligence {
            AiMethods::Qlearn(params) => {
                let foods: Vec<Position> = foods.iter().copied().collect();
                let mut all_obstacles: Vec<Position> = obstacles.iter().copied().collect();
                all_obstacles.extend(&arena_obstacles);


                println!("{:?}", params);

                let direction = params.run(*pos, foods, all_obstacles);

                head.direction = match direction as u8 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Right,
                    3 => Direction::Left,
                    _ => todo!()
                };
            }
            _ => {}
        }
    }
}
