use bevy::prelude::*;

use crate::game::ui::{game_over_menu::components::*, styles::*};

pub fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.single() {
        commands.entity(game_over_menu_entity).despawn();
    }
}

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((MENU_NODE_STYLE, GameOverMenu))
        .with_children(|parent| {
            // === Title ===
            parent.spawn(TITLE_NODE_STYLE).with_children(|parent| {
                // Image 1
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("sprites/ball_red_large.png"),
                        ..default()
                    },
                    GAME_OVER_IMAGE_NODE_STYLE,
                ));
                // Text
                parent.spawn((
                    Text::new("Game Over"),
                    title_text_font(&asset_server),
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: JustifyText::Center,
                        linebreak: LineBreak::NoWrap,
                    },
                ));
                // Image 2
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("sprites/ball_red_large.png"),
                        ..default()
                    },
                    GAME_OVER_IMAGE_NODE_STYLE,
                ));
            });
            // === Resume Button ===
            parent
                .spawn((
                    Button,
                    RestartButton,
                    BUTTON_NODE_STYLE,
                    BackgroundColor(NORMAL_BUTTON_COLOUR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Restart"),
                        button_text_font(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // === Main Menu Button ===
            parent
                .spawn((
                    Button,
                    MainMenuButton,
                    BUTTON_NODE_STYLE,
                    BackgroundColor(NORMAL_BUTTON_COLOUR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Menu"),
                        button_text_font(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            // === Quit Button ===
            parent
                .spawn((
                    Button,
                    QuitButton,
                    BUTTON_NODE_STYLE,
                    BackgroundColor(NORMAL_BUTTON_COLOUR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        button_text_font(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        });
}
