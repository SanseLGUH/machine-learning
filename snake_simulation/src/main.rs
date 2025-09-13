use bevy::{ window::*, prelude::*};

mod components;
mod plugins;
mod menu;

mod settings;
mod window;

use crate::plugins::snake::ai::qlearn::SnakeState;

use crate::components::*;
use crate::settings::*;

use std::collections::HashMap;

#[derive(Default, Resource)]
struct Qtable {
    qt: HashMap<(SnakeState, Direction), f64>
}

fn main() {
    App::new()
        // Default Plugins
        .add_plugins( DefaultPlugins.set( WindowPlugin {
            primary_window: Some( Window {
                title: String::from( "AI PLAY WITH MY SNAKE NOW! " ),
                resolution: WindowResolution::new( 1200., 700. ),
                ..default()
            } ),
            ..default()
        } ),  )   
        
        // EguiPlugin
        .add_plugins(
            menu::Settings
        )
        .add_plugins(( 
            plugins::Snake, 
            plugins::Apple 
        ))
        .register_type::<GameState>()
        .init_resource::<GameState>()

        // GameState, Positioning, Window_Scalling
        .insert_resource( GameState::default() )
        .insert_resource( Qtable::default() )
        .register_type::<GameState>()

        .add_plugins(ResourceInspectorPlugin::<GameState>::default())

        .add_systems( Startup, setup_camera )
        .add_systems( Update, ( window::size_scalling, window::position_translation ) )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn( Camera2d );
}
use bevy_inspector_egui::quick::ResourceInspectorPlugin;