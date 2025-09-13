use bevy::prelude::*;

use rurel::{
    mdp::{State, Agent},
    AgentTrainer, strategy::learn::{ QLearning, LearningStrategy}, 
    strategy::explore::{RandomExploration, ExplorationStrategy}, strategy::terminate::FixedIterations
};
use smart_default::SmartDefault;
use rand::prelude::*;

use std::collections::HashMap;

use crate::{Qtable, components::*, settings::*};

#[derive(Default, Clone, Hash, Eq, PartialEq, Debug)]
pub struct SnakeState {
    body: Vec<(i32, i32)>,
    direction: Direction,
    food: Vec<(i32, i32)>,
    obstacles: Vec<(i32, i32)>,
}

impl State for SnakeState {
    type A = Direction;

    fn reward(&self) -> f64 {
        // define reward: e.g. +1 if eat, -1 if die, etc.
        // For example:
        // if self.head == self.food {
        //     1.0
        // } else {
        //     0.0
        // }
        0.
    }

    fn actions(&self) -> Vec<Self::A> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

// Define your agent
#[derive(Default)]
pub struct SnakeAgent {
    pub state: SnakeState,
    pub epsilon: f64,
}

impl SnakeAgent {
    pub fn choose_action(&self, q_table: HashMap<(SnakeState, Direction), f64>) -> Direction {
        let mut rng = rand::rng();
        let actions = self.state.actions();

        if rng.r#gen::<f64>() < self.epsilon {
            let i = rng.gen_range(0..actions.len());
            return actions[i].clone();
        }

        // Exploitation: choose the action with the highest Q-value
        let mut best_action = actions[0].clone();
        let mut best_value = f64::MIN;

        for action in actions {
            let q_value = *q_table.get(&(self.state.clone(), action.clone())).unwrap_or(&0.0);
            if q_value > best_value {
                best_value = q_value;
                best_action = action;
            }
        }

        best_action
    }

    pub fn train_step(
        &mut self,
        mut q_table: HashMap<(SnakeState, Direction), f64>,
        learning_strategy: &QLearning,
        exploration_strategy: RandomExploration,
    ) -> Direction {
        let s_t = self.current_state().clone();
        let action = exploration_strategy.pick_action(self);

        let s_t_next = self.current_state().clone();
        let r_t_next = s_t_next.reward();

        // Get the old Q-value Q(s, a)

        let v = {
            let q_values_for_state: HashMap<Direction, f64> = q_table
                .iter()
                .filter_map(|((state, dir), value)| {
                    if state == &s_t {
                        Some((dir.clone(), *value))
                    } else {
                        None
                    }
                })
                .collect();


            let old_value = q_table.get(&(s_t.clone(), action.clone()));    
            <QLearning as LearningStrategy<SnakeState>>::value(
                learning_strategy,
                &Some(&q_values_for_state),
                &old_value,
                r_t_next,
            )
        };

        // Get the best future Q-value for s_t+1
        let future_values: Vec<f64> = 
            self.state.actions()
            .iter()
            .filter_map(|a| q_table.get(&(s_t_next.clone(), a.clone())).copied())
            .collect();


        // Update the Q-table
        q_table.insert((s_t, action), v);

        Direction::Up
    }

}

impl Agent<SnakeState> for SnakeAgent {
    fn current_state(&self) -> &SnakeState {
        &self.state
    }

    fn take_action(&mut self, action: &Direction) { 
        // update self.state based on the action
        // e.g., move head position, check collisions, maybe reset, etc.

        // am not using that bc bevy solved this problem
    }
}
