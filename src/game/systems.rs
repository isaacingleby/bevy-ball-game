use bevy::prelude::*;

use super::SimulationState;

/// Added due to tutorial, but not used in my version of the game.
pub fn _pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
}

/// Added due to tutorial, but not used in my version of the game.
pub fn _resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match **current_simulation_state {
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
                println!("Simulation Paused");
            }
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
                println!("Simulation Resumed");
            }
        }
    }
}
