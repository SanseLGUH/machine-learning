use bevy::prelude::*;
use crate::{*, components::*};

pub mod snake;

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
}
