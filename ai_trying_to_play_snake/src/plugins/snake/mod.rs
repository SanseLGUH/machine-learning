use bevy::prelude::*;

pub mod ai;

use crate::*;
use crate::components::*;

pub fn spawn_head(game_state: Res<GameState>, mut commands: Commands) {
	commands.spawn( (
		SnakeHead, Sprite::from_color( 
			Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(1., 1.) 
			// change Vec2 when starting write scalling func 
		), Position::random( game_state.arena_size.width, game_state.arena_size.height ), Size::squaire( 1. ), 
	) );
}

pub fn change_direction( mut game_state: ResMut<GameState> ) {
} 

pub fn eats( heads: Query< &Position, (With< SnakeHead >, Without< Eattable >) >, foods: Query< (Entity, &Position), With<Eattable> > ) {
}

pub fn growths( mut game_state: ResMut<GameState> ) {
}

pub fn dies( snake_heads: Query< Entity, With< SnakeHead > >, snake_segments: Query< Entity, With< SnakeSegment > > ) {
}
