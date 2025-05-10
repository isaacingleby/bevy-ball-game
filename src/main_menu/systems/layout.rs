use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((MAIN_MENU_NODE_STYLE, MainMenu))
        .with_children(|parent| {
            // === Title ===
            parent.spawn(TITLE_NODE_STYLE).with_children(|parent| {
                // Image 1
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("sprites/ball_blue_large.png"),
                        ..default()
                    },
                    IMAGE_NODE_STYLE,
                ));
                // Text
                parent.spawn((
                    Text::new("Bevy Ball Game"),
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
                    IMAGE_NODE_STYLE,
                ));
            });

            // === Play Button ===
            parent
                .spawn((
                    Button,
                    PlayButton,
                    BUTTON_NODE_STYLE,
                    BackgroundColor(COLOUR_NORMAL),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
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
                    BackgroundColor(COLOUR_NORMAL),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        button_text_font(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        })
        .id();
    main_menu_entity
}
