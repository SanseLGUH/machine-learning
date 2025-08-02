use bevy::prelude::*;
use crate::{components::*, settings::GameState};

pub mod ai;

#[derive(Bundle)]
struct SnakeHeadBundle {
	snake_head: Head,
	
	sprite: Sprite,
	size: Size,
	
	position: Position,
	direction: Direction,
}

pub fn spawn_head(game_state: Res<GameState>, mut commands: Commands) {
	commands.spawn(
		SnakeHeadBundle {
		    snake_head: Head,
		    sprite: Sprite::from_color(Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(1., 1.)),
		    size: Size::square(1.0),
		    position: Position::random(game_state.arena_size.width, game_state.arena_size.height),
		    direction: Direction::default()		
		}
	);

}

pub fn moves( time: Res<Time>, mut game_state: ResMut<GameState>, snakes: Query< ( &mut Position, &Direction ), (With<Head>, Without< Segment >) >, segments: Query< &mut Position, (With<Segment>, Without< Head >) > ) {
	if game_state.pause {
		return;
	} 

	for ( mut position, direction ) in snakes {
		if game_state.world_speed.snake.tick( time.delta() ).finished() {
			match direction {
				Direction::Up => position.y += 1,
				Direction::Down => position.y -= 1,
				Direction::Right => position.x += 1,
				Direction::Left => position.x -= 1
			}
		}
	}
}

pub fn eats( heads: Query< &Position, With< Head > >, foods: Query< (Entity, &Position), With<Eattable> > ) {
}

pub fn growths( mut game_state: ResMut<GameState> ) {
}

pub fn segments_follows_head( heads: Query< &mut Position, With< Head > >, mut segments: Query< &mut Position, With< Segment > > ) {
}

pub fn dies( game_state: Res<GameState>, snake_heads: Query< (Entity, &Position), With< Head > >, snake_segments: Query< Entity, With< Segment > >) {
	// for (entity, pos) in snake_heads {
	// 	for (entity, pos) in snake_segments {
	// 	}
	// }
}
			