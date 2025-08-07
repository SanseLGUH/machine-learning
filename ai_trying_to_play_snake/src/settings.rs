use bevy::prelude::*;
use smart_default::SmartDefault;

use crate::plugins::snake::ai::{Qlearning, Astar, ModifiedAstar};

#[derive(Reflect, SmartDefault, PartialEq, Debug)]
pub enum AiMethods {
    Astar( Astar ),
    ModifiedAstar( ModifiedAstar ),

    #[default]
    Qlearn( Qlearning )
}

use std::fmt;

impl fmt::Display for AiMethods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiMethods::Astar(_) => write!(f, "Astar"),
            AiMethods::ModifiedAstar(_) => write!(f, "ModifiedAstar"),
            AiMethods::Qlearn(_) => write!(f, "Qlearn"),
        }
    }
}

#[derive(Reflect, SmartDefault)]
pub struct ArenaSize {
    #[default = 50]
    pub height: i32,

    #[default = 85]
    pub width: i32
}

#[derive(SmartDefault)] 
pub struct WorldLimitations {
    #[default = 1]
    snakes: u64,

    #[default = 1]
    foods: u64
}

#[derive(Reflect, SmartDefault)]
pub struct WorldSpeed {
    #[default(_code = "Timer::from_seconds( 0.3, TimerMode::Repeating)")]
    pub snake: Timer,
    #[default(_code = "Timer::from_seconds( 3., TimerMode::Repeating)")]
    pub food_spawn: Timer
} 

#[derive(Reflect, Resource, SmartDefault)]
pub struct GameState {
    #[default = false]
    pub menu_state: bool,
    
    pub arena_size: ArenaSize,
    pub ai_method: AiMethods,

    pub world_limit: WorldLimitations,

    pub world_speed: WorldSpeed,
    
    #[default = true]
    pub pause: bool
}