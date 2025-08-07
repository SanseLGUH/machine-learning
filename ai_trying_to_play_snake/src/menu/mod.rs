use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};

use bevy::prelude::*;
use bevy_egui::{
	egui, EguiContexts, EguiPlugin, 
	EguiPrimaryContextPass
};

use crate::plugins::snake::ai::*;
use crate::settings::{AiMethods, GameState};

pub struct Settings;

impl Plugin for Settings {
	fn build(&self, app: &mut App) {
		app
			.add_plugins( ( bevy_egui::EguiPlugin::default(), ResourceInspectorPlugin::<GameState>::default(), WorldInspectorPlugin::new()) )
			.add_systems( EguiPrimaryContextPass, egui_setup )
			.add_systems( Update, update_state );
	}
}

// #[derive(Reflect, SmartDefault)]
// pub enum AiMethods {
//     Astar( Astar ),
//     ModifiedAstar( ModifiedAstar ),

//     #[default]
//     Qlearn( Qlearning )
// }  

fn egui_setup(mut contexts: EguiContexts, mut game_state: ResMut<GameState> ) -> Result {
	if game_state.menu_state == false {
		return Ok(());
	}
	
	egui::Window::new("WorldSettings").show(contexts.ctx_mut()?, |ui| {
		egui::ComboBox::from_label("Select one!")
		    .selected_text( format!( "{}", game_state.ai_method ) )
		    .show_ui(ui, |ui| {
		        ui.selectable_value(&mut game_state.ai_method, AiMethods::Qlearn( Qlearning::default() ), "Qlearning");
		        ui.selectable_value(&mut game_state.ai_method, AiMethods::Astar( Astar::default() ), "Astar");
		        ui.selectable_value(&mut game_state.ai_method, AiMethods::ModifiedAstar( ModifiedAstar::default() ), "ModifiedAstar");
		    }
		);

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