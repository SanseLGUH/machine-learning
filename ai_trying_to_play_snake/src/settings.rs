use bevy::prelude::*;
use smart_default::SmartDefault;

#[derive(Default)]
pub enum AiMethods {
    Astar, 
    ModifiedAstar, 
    #[default]
    QLearningAI 
}  

#[derive(SmartDefault)]
pub struct ArenaSize {
    #[default = 50]
    pub height: u8,

    #[default = 85]
    pub width: u8
}

pub struct WorldSpeed {
    pub snake: Timer,
    pub food_spawn: Timer
} 

impl Default for WorldSpeed {
    fn default() -> Self {
        Self {
            snake: Timer::from_seconds( 0.3, TimerMode::Repeating),
            food_spawn: Timer::from_seconds( 3., TimerMode::Repeating)
        }
    }
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