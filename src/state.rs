use bevy::prelude::States;

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

#[cfg(test)]
mod tests {
    use bevy::{DefaultPlugins, app::App, state::app::AppExtStates};

    use crate::state::GameState;

    #[test]
    fn test_init() {
        let mut app = App::new();
        app.init_state::<GameState>();
    }
}
