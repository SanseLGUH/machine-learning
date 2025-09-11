use bevy::prelude::*;
use crate::{components::*, settings::{GameState, ArenaSize}};

pub mod ai;

#[derive(Bundle)]
struct SnakeHeadBundle {
	head: Head,
	sprite: Sprite,
	size: Size,
	position: Position
}

pub fn spawn_head(
	snakes: Query< &Head >,  
	game_state: Res<GameState>, 
	mut commands: Commands
) {
	if snakes.iter().len() >= game_state.world_limit.snakes as usize {
		return;
	}

	let iterate_times = game_state.world_limit.snakes as usize - snakes.iter().len();

	for _ in 0..iterate_times {
		commands.spawn(
			SnakeHeadBundle {
			    head: Head::default(),
			    sprite: Sprite::from_color(Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(1., 1.)),
			    size: Size::square(1.0),
			    position: Position::random(game_state.arena_size.width, game_state.arena_size.height)
			}
		);
	}
}

pub fn growths( 
	mut commands: Commands, 
	game_state: Res<GameState>, 
	mut event: EventReader<GrowthEvent> 
) {
	for growth in event.read() {
		commands.spawn((
			Segment::with_owner( growth.owner_index, growth.index ), Obstacle, 
			Sprite::from_color(Color::srgb( 0.80, 0.49, 0.12 ), Vec2::new(1., 1.)),
			Size::square( 0.8 ), Position::new(0, 0)
		));    
    }
}

pub fn moves(
	game_state: Res<GameState>,
    time: Res<Time>,
    mut heads: Query<(Entity, &mut Head, &mut Position), Without<Segment>>,
    mut segments: Query<(&Segment, &mut Position), Without<Head>>,
) {
	if game_state.pause {
		return;
	}

    for (entity, mut head, mut head_pos) in heads.iter_mut() {
        if head.timer.tick(time.delta()).just_finished() {
            let old_head_pos = *head_pos;

            position_calc(&mut head_pos, &head.direction);

            let mut snake_segments: Vec<_> = segments
                .iter_mut()
                .filter(|(segment, _)| segment.owner_index == entity.index())
                .collect();
            snake_segments.sort_by_key(|(segment, _)| segment.index);

            let mut previous_positions = vec![old_head_pos];
            for (_, pos) in &snake_segments {
                previous_positions.push(**pos);
            }

            for ((_, segment_pos), new_pos) in snake_segments.iter_mut().zip(previous_positions.iter()) {
                **segment_pos = *new_pos;
            }
        }
    }
}


fn position_calc( pos: &mut Position, direction: &Direction ) {
	match direction {
		Direction::Up => pos.y += 1,
		Direction::Down => pos.y -= 1,
		Direction::Right => pos.x += 1,
		Direction::Left => pos.x -= 1
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
				
				growth_writer.send(
					GrowthEvent::new( 
						head_entity.index(), head.eat_count
					)
				);
			}
		}
	}

}

pub fn dies( 
    mut commands: Commands, 
    game_state: Res<GameState>, 
    heads: Query<(Entity, &Position), With<Head>>, 
    segments: Query<(Entity, &Segment)>, 
    obstacles: Query<&Position, With<Obstacle>>
) {
    fn despawn_snake(commands: &mut Commands, head: Entity, segments: &Query<(Entity, &Segment)>) {
        for (entity, segment) in segments.iter() {
            if segment.owner_index == head.index() {
                commands.entity(entity).despawn();
            }
        }
        commands.entity(head).despawn();
    }

    for (head, pos) in heads.iter() {
        if out_of_bounds(pos, &game_state.arena_size) 
            || obstacles.iter().any(|ob_pos| *pos == *ob_pos) 
        {
            despawn_snake(&mut commands, head, &segments);
        }
    }
}

fn out_of_bounds(pos: &Position, arena_size: &ArenaSize) -> bool {
    pos.x < 0 || pos.y < 0 || pos.x >= arena_size.width || pos.y >= arena_size.height
}
