use bevy::prelude::*;
use bevy_egui::{
	egui, EguiContexts, EguiPlugin, 
	EguiPrimaryContextPass
};

use crate::GameState;

pub struct Settings;

impl Plugin for Settings {
	fn build(&self, app: &mut App) {
		app
			.add_systems( EguiPrimaryContextPass, egui_setup )
			.add_systems( Update, update_state );
	}
}

fn egui_setup(mut contexts: EguiContexts, game_state: Res<GameState> ) -> Result {
	if game_state.menu_state == false {
		return Ok(());
	}
	

	// 
	egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });

	Ok(())
} 

fn update_state( mut game_state: ResMut<GameState>, keys: Res<ButtonInput<KeyCode>> ) {
	if keys.just_pressed(KeyCode::Escape) {
		game_state.menu_state = !game_state.menu_state;
		println!("pressed");
	}
}