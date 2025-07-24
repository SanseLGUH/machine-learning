use bevy::{ window::*, prelude::*};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;

mod components;
mod plugins;

use crate::{
    components::*, plugins::*
};

#[derive(Reflect)]
enum AiMethods {
    Astar, ModifiedAstar, QLearningAI 
}  

impl Default for AiMethods {
    fn default() -> Self {
        Self::QLearningAI
    }
}

struct Snake {
    current_direction: Direction, 
    spawn_pos: [ u8; 2 ],
}

#[derive(Reflect)]
struct ArenaSize {
    height: u8,
    width: u8
}

impl Default for ArenaSize {
    fn default() -> Self {
        Self {
            height: 50,
            width: 50  
        }
    }
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct GameState {
    arena_size: ArenaSize,

    // snake: Option< Snake >,
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
        } ),  )   
        
        // EguiPlugin
        .add_plugins(( EguiPlugin::default(), WorldInspectorPlugin::new(), ResourceInspectorPlugin::<GameState>::default() ))
        .add_plugins( plugins::Snake )
        .init_resource::<GameState>()
        .register_type::<GameState>()

        .insert_resource( GameState::default() )
        .add_systems( Startup, setup_camera )
        .add_systems( Update, ( size_scalling, position_translation ) )
        .run();
}

fn size_scalling( game_state: Res<GameState>, windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)> ) {
    let window = windows.single().unwrap();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            ( sprite_size.width as f32 / game_state.arena_size.width as f32 * window.width() ) as f32,
            ( sprite_size.height as f32 / game_state.arena_size.height as f32 * window.height() ) as f32,
            1.,
        );
    }
}

fn position_translation( game_state: Res<GameState>, windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)> ) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.single().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, game_state.arena_size.width as f32),
            convert(pos.y as f32, window.height() as f32, game_state.arena_size.height as f32),
            0.0,
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn( Camera2d );
}
