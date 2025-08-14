use bevy::{prelude::*, window::*};
use crate::components::{Position, Size};
use crate::settings::{GameState};


pub fn size_scalling( game_state: Res<GameState>, windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)> ) {
    let window = windows.single().unwrap();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            ( sprite_size.width as f32 / game_state.arena_size.width as f32 * window.width() ) as f32,
            ( sprite_size.height as f32 / game_state.arena_size.height as f32 * window.height() ) as f32,
            1.,
        );
    }
}

pub fn position_translation( game_state: Res<GameState>, windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)> ) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.single().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, game_state.arena_size.width as f32),
            convert(pos.y as f32, window.height() as f32, game_state.arena_size.height as f32),
            0.0,
        );
    }
}