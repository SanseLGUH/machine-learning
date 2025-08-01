use bevy::prelude::*;

pub mod utils;

use crate::components::*;
use crate::settings::*;

pub fn controller( mut game_state: ResMut<GameState>, snake_heads: Query< (&Position, &mut Direction), With<SnakeHead>> )  {
	for ( pos, mut direction ) in snake_heads {
		*direction = Direction::Left;
	}
}