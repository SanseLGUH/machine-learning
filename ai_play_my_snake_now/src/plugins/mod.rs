use bevy::prelude::*;
use crate::{*, components::*};

pub mod snake;

pub struct Snake;

impl Plugin for Snake {
	fn build( &self, app: &mut App ) {
		app.add_systems( Update, (
			snake::spawn_head, snake::ai::controller, snake::moves , snake::dies, snake::eats, 
			snake::growths.after( snake::eats ) 
		));
		app.add_event::<GrowthEvent>();
	}
}

pub struct Apple;

impl Plugin for Apple {
	fn build( &self, app: &mut App ) {
		app.add_systems( Update, spawn_food );
	}
}

fn spawn_food(mut commands: Commands, time: Res<Time>, foods: Query< (Entity, &Eattable) >,  mut game_state: ResMut<GameState>) {
	let foods_count = foods.iter().len();

	if foods_count > game_state.world_limit.foods as usize {
		for (en, _) in foods {
			commands.entity( en ).despawn();	
		}
	}

	if game_state.pause {
		return;
	}

	if foods_count < game_state.world_limit.foods as usize && game_state.world_speed.food_spawn.tick( time.delta() ).finished() {	
		commands.spawn((
			Eattable, Sprite::from_color(Color::srgb( 1., 1., 1. ), Vec2::new(1., 1.)), Size::square( 1. ),
			Position::random(game_state.arena_size.width, game_state.arena_size.height)
		));
	}
}
