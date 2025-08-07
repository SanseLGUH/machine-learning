use bevy::prelude::*;
use crate::{components::*, settings::{GameState, ArenaSize}};

pub mod ai;

#[derive(Bundle)]
struct SnakeHeadBundle {
	snake_head: Head,
	sprite: Sprite,
	size: Size,
	position: Position,
	direction: Direction
}

pub fn spawn_head( 
	game_state: Res<GameState>, 
	mut commands: Commands
) {

	commands.spawn(
		SnakeHeadBundle {
		    snake_head: Head::default(),
		    sprite: Sprite::from_color(Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(1., 1.)),
		    size: Size::square(1.0),
		    position: Position::random(game_state.arena_size.width, game_state.arena_size.height),
		    direction: Direction::default(),
		}
	);

}

// #[derive(Default)]
// pub struct Tail {
// 	pub tail_size: Vec< Entity >,
// 	pub last_tail_position: Option< Position >
// }

// #[derive(Component, Default)]
// pub struct Head {
// 	pub tail: Tail
// }


pub fn moves( 
	time: Res<Time>, 
	mut game_state: ResMut<GameState>, 
	heads: Query< ( Entity, &mut Head, &mut Position, &Direction ), Without< Segment > >, 
	mut segments: Query< (&Segment, &mut Position), Without< Head > > 
) {
	if game_state.pause {
		return;
	} 

	for ( entity, mut head, mut position, direction ) in heads {
		if game_state.world_speed.snake.tick( time.delta() ).finished() {
			
			head.tail.last_position = Some( position.clone() );

			for ( segment, mut pos ) in &mut segments {
				if entity.index() == segment.owner_index {
					*pos = position.clone();
				}
			}

			match direction {
				Direction::Up => position.y += 1,
				Direction::Down => position.y -= 1,
				Direction::Right => position.x += 1,
				Direction::Left => position.x -= 1
			}

			// for ( segment_entity, mut pos ) in &mut segments {
			// 	let index = segment_entity.index();

			// 	for index_from_tail in head.tail.indexes.clone() {
			// 		if index_from_tail == index {
			// 			*pos = position.clone();
			// 		}
			// 	}
			// }
		}
	}
}

pub fn eats(  
	mut commands: Commands, 
	heads: Query< (Entity, &Head, &Position) >, 
	foods: Query< (Entity, &Position), (With<Eattable>, Without< Head >) >, 
	mut growth_writer: EventWriter<GrowthEvent> 
) {
	for ( head_entity, head, head_pos) in heads {
		for ( en, pos ) in foods {
			if head_pos == pos {
				commands.entity( en ).despawn();
				growth_writer.send(GrowthEvent::new( head_entity.index(), head.tail.last_position.clone().unwrap() ));
			}
		}
	}

}

pub fn growths( 
	mut commands: Commands, 
	game_state: Res<GameState>, 
	mut event: EventReader<GrowthEvent> 
) {
	for growth in event.read() {
		commands.spawn((
			Segment::with_owner( growth.owner_index ), Obstacle, Sprite::from_color(Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(1., 1.)),
			Size::square( 0.8 ), growth.owner_position.clone()	
		));    
    }
}

pub fn dies( 
	mut commands: Commands, 
	game_state: Res<GameState>, 
	heads: Query< (Entity, &Position), With< Head > >, segments: Query< (Entity, &Segment)>, 
	obstacle: Query< &Position, With< Obstacle > >
) {
    for (entity, pos) in heads {
    	for ob_pos in obstacle {
    		if pos == ob_pos || out_of_bounds( pos, &game_state.arena_size ) {

    			for ( en, segment ) in segments {
    				if entity.index() == segment.owner_index {
    					commands.entity( en ).despawn();
    				}
    			}

    			commands.entity( entity ).despawn();
    		}

    	}
    }
}

fn out_of_bounds(pos: &Position, arena_size: &ArenaSize) -> bool {
    pos.x < 0 || pos.y < 0 || pos.x >= arena_size.width || pos.y >= arena_size.height
}
