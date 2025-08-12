use std::ops::Not;

use crate::imports::*;
use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                toggle_simulation.run_if(in_state(AppState::Game)),
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        );
    }
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.eq(&SimulationState::Running) {
            commands.insert_resource(NextState::Pending(SimulationState::Paused));
            println!("Simulation paused");
        } else {
            commands.insert_resource(NextState::Pending(SimulationState::Running));
            println!("Simulation continued");
        }
    }
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if app_state.get().ne(&AppState::Game) {
            commands.insert_resource(NextState::Pending(AppState::Game));
            println!("Transitioning to game state")
        }
        //  else {
        //     commands.insert_resource(NextState::Pending(AppState::MainMenu));
        // }
    }
}
pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if app_state.get().ne(&AppState::MainMenu) {
            commands.insert_resource(NextState::Pending(AppState::MainMenu));
            println!("Transitioning to main menu state")
        }
        //  else {
        //     commands.insert_resource(NextState::Pending(AppState::MainMenu));
        // }
    }
}
