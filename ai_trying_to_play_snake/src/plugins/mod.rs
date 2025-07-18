use bevy::prelude::*;
use crate::components::*;

mod snake;

pub struct Snake;
impl Plugin for Snake {
	fn build( &self, app: &mut App ) {
	}
}

#[derive(Component)]
pub struct Food;

fn spawn_food(mut commands: Commands) {
}

impl Plugin for Food {
	fn build( &self, app: &mut App ) {
	}
} 