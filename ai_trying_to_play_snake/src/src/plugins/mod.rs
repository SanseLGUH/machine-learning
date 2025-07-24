use bevy::prelude::*;
use crate::{*, components::*};

mod snake;

pub struct Snake;

impl Plugin for Snake {
	fn build( &self, app: &mut App ) {
		app.add_systems( Startup, snake::spawn_head );
		app.add_systems( Update, (
			snake::ai::controller, snake::dies, snake::eats, 
			snake::growths.after( snake::eats ) 
		));
	}
}

#[derive(Component)]
pub struct Food;

impl Plugin for Food {
	fn build( &self, app: &mut App ) {
		app.add_systems( Update, spawn_food );
	}
}

fn spawn_food(mut commands: Commands, time: Res<Time>, mut game_state: ResMut<GameState>) {
	if game_state.food_spawn_rate.tick( time.delta() ).finished() {
		commands.spawn((
			Sprite::from_color( Color::srgb( 0., 0., 0. ), Vec2::new( 1., 1. ) ), Size::squaire( 1. ),
			Position::random( game_state.arena_size.width, game_state.arena_size.height )
		));
	}
}
