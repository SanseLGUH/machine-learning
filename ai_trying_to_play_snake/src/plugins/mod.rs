use bevy::prelude::*;
use crate::{*, components::*};

mod snake;

pub struct Snake;

impl Plugin for Snake {
	fn build( &self, app: &mut App ) {
		app.add_systems( Startup, snake::spawn_head );
		app.add_systems( Update, (
			snake::ai::controller, snake::moves , snake::dies, snake::eats, 
			snake::growths.after( snake::eats ) 
		));
	}
}

pub struct Apple;

impl Plugin for Apple {
	fn build( &self, app: &mut App ) {
		app.add_systems( Update, spawn_food );
	}
}

fn spawn_food(mut commands: Commands, time: Res<Time>, foods: Query< &Eattable >,  mut game_state: ResMut<GameState>) {
	if foods.iter().len() >= game_state.food_at_once.into() || game_state.pause {
		return;
	}  

	// if game_state.food_spawn_rate.tick( time.delta() ).finished() {
	// 	// commands.spawn((
	// 	// 	// Eattable, 
	// 	// 	// Sprite::from_color( Color::srgb( 0.39, 0.44, 0.15 ), Vec2::new( 1., 1. ) ), 
	// 	// 	// Size::square( 0.8 ),
	// 	// 	// Position::random( game_state.arena_size.width, game_state.arena_size.height ),
	// 	// ));
	// }
}
