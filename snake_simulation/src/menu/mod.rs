use bevy::prelude::*;
use bevy_egui::{
	egui, EguiContexts, EguiPlugin, 
	EguiPrimaryContextPass
};

use crate::components::Direction;
use crate::plugins::snake::ai::*;
use crate::settings::GameState;

pub struct Settings;

impl Plugin for Settings {
	fn build(&self, app: &mut App) {
		app
			.add_plugins( (bevy_egui::EguiPlugin::default(), bevy_inspector_egui::quick::WorldInspectorPlugin::new()) )
			.add_systems( EguiPrimaryContextPass, egui_setup )
			.add_systems( Update, update_state );
	}
}

fn egui_setup(mut contexts: EguiContexts, mut game_state: ResMut<GameState>, directions: Query<&mut Direction> ) -> Result {
	if game_state.menu_state == false {
		return Ok(());
	}

	egui::Window::new("WorldSettings").show(contexts.ctx_mut()?, |ui| {

		ui.toggle_value( &mut game_state.pause, "Pause" );

		// egui::ComboBox::from_label("Select one!")
		//     .selected_text( format!( "{}", game_state.ai_method ) )
		//     .show_ui(ui, |ui| {
		//         ui.selectable_value(&mut game_state.ai_method, AiMethods::Qlearn( Qlearning::default() ), "Qlearning");
		//         ui.selectable_value(&mut game_state.ai_method, AiMethods::Astar( Astar::default() ), "Astar");
		//         ui.selectable_value(&mut game_state.ai_method, AiMethods::ModifiedAstar( ModifiedAstar::default() ), "ModifiedAstar");
		//     }
		// );

		ui.horizontal( |ui| {
			ui.add( egui::DragValue::new(&mut game_state.world_limit.snakes).speed(0.1) );
			ui.add( egui::DragValue::new(&mut game_state.world_limit.foods).speed(0.1) );
			ui.label(" Snakes / Foods ");
		});

		ui.horizontal( |ui| {
			// ui.add_enabled( false, egui::DragValue::new(&mut game_state.world_speed.snake.duration().as_secs_f32()) );
			ui.add_enabled( false, egui::DragValue::new(&mut game_state.world_speed.food_spawn.duration().as_secs_f32()) );
		});

        ui.add(egui::Slider::new(&mut game_state.arena_size.width, 0..=120).text("Arena width"));
        ui.add(egui::Slider::new(&mut game_state.arena_size.height, 0..=120).text("Arena height"));
    });

	Ok(())
} 

fn update_state( mut game_state: ResMut<GameState>, keys: Res<ButtonInput<KeyCode>> ) {
	if keys.just_pressed(KeyCode::Escape) {
		game_state.menu_state = !game_state.menu_state;
	}
}