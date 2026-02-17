use bevy::prelude::*;

use crate::{state::GameState, ui};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_systems(OnEnter(GameState::Menu), ui::main_menu::setup_main_menu)
        .add_systems(
            Update,
            ui::main_menu::main_menu_button_system.run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), ui::main_menu::cleanup_main_menu)
        .run();
}
