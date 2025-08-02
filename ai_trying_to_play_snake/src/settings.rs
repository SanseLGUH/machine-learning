use bevy::prelude::*;
use smart_default::SmartDefault;

use crate::plugins::snake::ai::Qlearning;

#[derive(SmartDefault)]
pub enum AiMethods {
    Astar { 
        params: [ f64; 3 ] 
    }, 
    ModifiedAstar {
        params: [ f64; 3],

        // William Rowan Hamilton Strategy
        wrh_strategy: [ u8; 510 ]
    }, 

    #[default]
    Qlearning {
        weights: [ f64; 3 ],
    }
}  

#[derive(SmartDefault)]
pub struct ArenaSize {
    #[default = 50]
    pub height: u8,

    #[default = 85]
    pub width: u8
}

#[derive(SmartDefault)]
pub struct WorldSpeed {
    #[default(_code = "Timer::from_seconds( 0.3, TimerMode::Repeating)")]
    pub snake: Timer,
    #[default(_code = "Timer::from_seconds( 3., TimerMode::Repeating)")]
    pub food_spawn: Timer
} 

#[derive(Resource, SmartDefault)]
pub struct GameState {
    #[default = false]
    pub menu_state: bool,
    
    pub arena_size: ArenaSize,
    pub ai_method: AiMethods,

    #[default = 1]
    pub food_at_once: u8,

    pub world_speed: WorldSpeed,
    
    #[default = true]
    pub pause: bool
}