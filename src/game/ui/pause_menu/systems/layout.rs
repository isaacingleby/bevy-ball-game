use bevy::prelude::*;

use crate::game::ui::pause_menu::{components::*, styles::*};

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.single() {
        commands.entity(pause_menu_entity).despawn();
    }
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((PAUSE_MENU_NODE_STYLE, PauseMenu))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(PAUSE_TITLE_NODE_STYLE)
                .with_children(|parent| {
                    // Image 1
                    parent.spawn((
                        ImageNode {
                            image: asset_server.load("sprites/star.png"),
                            ..default()
                        },
                        PAUSE_IMAGE_NODE_STYLE,
                    ));
                    // Text
                    parent.spawn((
                        Text::new("Game Paused"),
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
                            image: asset_server.load("sprites/star.png"),
                            ..default()
                        },
                        PAUSE_IMAGE_NODE_STYLE,
                    ));
                });
            // === Resume Button ===
            parent
                .spawn((
                    Button,
                    ResumeButton,
                    PAUSE_BUTTON_NODE_STYLE,
                    BackgroundColor(NORMAL_BUTTON_COLOUR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Resume"),
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
                    PAUSE_BUTTON_NODE_STYLE,
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
                    PAUSE_BUTTON_NODE_STYLE,
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
