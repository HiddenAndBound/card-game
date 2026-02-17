use crate::state::GameState;
use bevy::{
    ecs::system::Commands,
    prelude::{ButtonInput, KeyCode, NextState, Res, ResMut},
    ui::Node,
};


pub fn setup_run_placeholder(mut commands: Commands) {}

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
