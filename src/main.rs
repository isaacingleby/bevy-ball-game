use bevy::prelude::*;
use bevy_ball_game::{
    game::{GamePlugin, states_::AppState},
    main_menu::MainMenuPlugin,
    runtime::dx12_plugin,
    systems::{transistion_to_game_state, transistion_to_main_menu_state},
};

fn main() {
    App::new()
        .add_plugins(dx12_plugin())
        .init_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(
            Update,
            (transistion_to_game_state, transistion_to_main_menu_state),
        )
        .run();
}
