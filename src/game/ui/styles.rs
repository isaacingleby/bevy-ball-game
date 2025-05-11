use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOUR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOUR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOUR: Color = Color::srgb(0.35, 0.35, 0.35);

pub const MENU_NODE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node.row_gap = Val::Px(8.0);
    node.column_gap = Val::Px(8.0);
    node
};

pub const TITLE_NODE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.flex_direction = FlexDirection::Row;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(300.0);
    node.height = Val::Px(120.0);
    node
};

pub const BUTTON_NODE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(200.0);
    node.height = Val::Px(80.0);
    node
};

pub const PAUSE_IMAGE_NODE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(32.0);
    node.height = Val::Px(32.0);
    node.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    node
};

pub const GAME_OVER_IMAGE_NODE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(64.0);
    node.height = Val::Px(64.0);
    node.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    node
};

pub fn title_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        ..default()
    }
}

pub fn button_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        ..default()
    }
}
