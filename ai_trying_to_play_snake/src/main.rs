use bevy::{ window::*, prelude::*};

mod components;
mod plugins;
mod menu;

mod settings;
mod window;

use crate::components::*;
use crate::settings::*;

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
        .add_plugins( ( menu::Settings, bevy_egui::EguiPlugin::default() ) )
        .add_plugins( (plugins::Snake, plugins::Apple) )
        .init_resource::<GameState>()

        // GameState, Positioning, Window_Scalling
        .insert_resource( GameState::default() )
        .add_systems( Startup, setup_camera )
        .add_systems( Update, ( window::size_scalling, window::position_translation, food_limitation ) )
        .run();
}



fn food_limitation( game_state: Res< GameState >, foods: Query< Entity, With<Eattable> > ) {
    if foods.iter().len() >= game_state.food_at_once.into() {
        // TODO
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn( Camera2d );
}
