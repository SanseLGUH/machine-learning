use bevy::prelude::*;

mod ai;
mod algorithms;

use crate::*;
use crate::components::*;

pub fn spawn_head(mut commands: Commands) {
	println!(" snake_head spawned "); 	

	commands.spawn( (
		Sprite::from_color( 
			Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(100., 100.) 
			// change Vec2 when starting write scalling func 
		), Position::random()
	) );
}

pub fn change_direction( mut game_state: ResMut<GameState> ) {
} 

pub fn crawls(mut commands: Commands) {
}

pub fn eats() {
}

pub fn growths() {
}

pub fn dies() {
}
