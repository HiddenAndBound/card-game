use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct StartButton;

const BUTTON_NORMAL: Color = Color::srgb(0.20, 0.20, 0.24);
const BUTTON_HOVERED: Color = Color::srgb(0.30, 0.30, 0.35);
const BUTTON_PRESSED: Color = Color::srgb(0.15, 0.55, 0.25);

pub fn setup_main_menu(mut commands: Commands) {
    // This is the background entity.
    commands
        .spawn((
            MainMenuRoot,
            Node {
                height: Val::Percent(100.00),
                width: Val::Percent(100.00),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.3, 0.5)),
        ))
        .with_children(|parent| {
            // Spawn the title
            parent.spawn((
                Text::new("Card Game"),
                TextFont {
                    font_size: 32.00,
                    weight: FontWeight::EXTRA_BOLD,
                    ..default()
                },
            ));

            // Here we spawn the buttons as children of the main menu background entity.
            parent
                .spawn((
                    Node {
                        justify_self: JustifySelf::Center,
                        height: Val::Percent(10.00),
                        width: Val::Percent(15.00),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BackgroundColor(BUTTON_NORMAL),
                ))
                .with_child((
                    Text::new("Start Run"),
                    TextFont {
                        font_size: 15.00,
                        weight: FontWeight::MEDIUM,
                        ..default()
                    },
                    TextColor(BUTTON_PRESSED),
                ));
        });
}

pub fn main_menu_button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED.into();
                next_state.set(GameState::Run);
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVERED.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL.into();
            }
        }
    }
}

pub fn cleanup_main_menu(mut commands: Commands, menus: Query<Entity, With<MainMenuRoot>>) {
    for menu in &menus {
        commands.entity(menu).despawn();
    }
}
