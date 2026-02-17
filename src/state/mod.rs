mod menu;

use bevy::{app::App, prelude::States};

// Game state definitions and transitions.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Pause,
    Run,
    Combat,
    Victory,
    Defeat,
}

// This function adds all state change related systems to the App.
fn setup_states(app: &mut App) {}
