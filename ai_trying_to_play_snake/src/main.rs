use bevy::{ window::*, prelude::*};

mod components;
mod plugins;

use crate::{
    components::*, plugins::*
};

fn main() {
    App::new()
        .add_plugins( DefaultPlugins.set( WindowPlugin {
            primary_window: Some( Window {
                title: String::from( " AI PLAY WITH MY SNAKE NOW! " ),
                resolution: WindowResolution::new( 700., 700. ),
                ..default()
            } ),
            ..default()
        } ) )
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
