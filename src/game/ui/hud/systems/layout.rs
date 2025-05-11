use bevy::prelude::*;

use crate::game::ui::{hud::components::*, styles::*};

pub fn despawn_hud_overlay(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<HUDOverlay>>,
) {
    if let Ok(game_over_menu_entity) = game_over_menu_query.single() {
        commands.entity(game_over_menu_entity).despawn();
    }
}

pub fn spawn_hud_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((HUD_NODE_STYLE, HUDOverlay))
        .with_children(|parent| {
            // === Star Counter ===
            parent
                .spawn((LHS_NODE_STYLE, BACKGROUND_COLOUR))
                .with_children(|parent| {
                    // Star Image
                    parent.spawn((
                        ImageNode {
                            image: asset_server.load("sprites/star.png"),
                            ..default()
                        },
                        GAME_OVER_IMAGE_NODE_STYLE,
                    ));
                    // Score Text
                    parent.spawn((
                        Text::new("0"),
                        title_text_font(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        ScoreText,
                    ));
                });
            // === Enemy Counter ===
            parent
                .spawn((RHS_NODE_STYLE, BACKGROUND_COLOUR))
                .with_children(|parent| {
                    // === Enemy Text ===
                    parent.spawn((
                        Text::new("0"),
                        title_text_font(&asset_server),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        EnemyText,
                    ));
                    // === Enemy Image ===
                    parent.spawn((
                        ImageNode {
                            image: asset_server.load("sprites/ball_red_large.png"),
                            ..default()
                        },
                        GAME_OVER_IMAGE_NODE_STYLE,
                    ));
                });
        });
}
