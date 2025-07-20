use bevy::{ window::*, prelude::*};

mod components;
mod plugins;

use crate::{
    components::*, plugins::*
};

enum AiMethods {
    Astar, ModifiedAstar,
    QLearningAI, 
}  

impl Default for AiMethods {
    fn default() -> Self {
        AiMethods::QLearningAI
    }
}

struct Snake {
    head_pos: Position,
    segments_pos: Vec< Position >,
    current_direction: Direction 
}

#[derive(Resource, Default)]
struct GameState {
    snake: Option< Snake >,
    ai_method: AiMethods,

    food_spawn_rate: Timer,
    food_at_once: u8,

    world_speed: f64,
    pause: bool
}

fn main() {
    App::new()
        .add_plugins( DefaultPlugins.set( WindowPlugin {
            primary_window: Some( Window {
                title: String::from( "AI PLAY WITH MY SNAKE NOW! " ),
                resolution: WindowResolution::new( 700., 700. ),
                ..default()
            } ),
            ..default()
        } ) )   
        .add_plugins( plugins::Snake )
        .insert_resource( GameState::default() )
        .add_systems( Startup, setup_camera )
        .add_systems( Update, ( size_scalling, position_translation ) )
        .run();
}

const ARENA_WIDTH: u8 = u8::MAX;
const ARENA_HEIGHT: u8 = u8::MAX;

fn size_scalling( windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)> ) {
    
}

fn position_translation( windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)> ) {

}

fn setup_camera(mut commands: Commands) {
    commands.spawn( Camera2d );
}
