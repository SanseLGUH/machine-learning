use bevy::prelude::*;
use smart_default::SmartDefault;

#[derive(Reflect, SmartDefault)]
pub struct QlearnScores {
    pub weights: [f64; 3],
    pub eat_count: u32
}

#[derive(Reflect, SmartDefault)]
pub struct ArenaSize {
    #[default = 50]
    pub height: i32,

    #[default = 85]
    pub width: i32
}

#[derive(Reflect, SmartDefault)] 
pub struct WorldLimitations {
    #[default = 1]
    pub snakes: u64,

    #[default = 1]
    pub foods: u64
}

#[derive(Reflect, SmartDefault)]
pub struct WorldSpeed {
    #[default(_code = "Timer::from_seconds( 3., TimerMode::Repeating)")]
    pub food_spawn: Timer
} 

#[derive(Reflect, Resource, SmartDefault)]
pub struct GameState {
    #[default = false]
    pub menu_state: bool,
    
    pub arena_size: ArenaSize,
    pub best_qlearn: QlearnScores,

    pub world_limit: WorldLimitations,

    pub world_speed: WorldSpeed,
    
    #[default = true]
    pub pause: bool
}