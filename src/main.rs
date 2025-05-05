use bevy::prelude::App;
use bevy_ball_game::{game::GamePlugin, main_menu::MainMenuPlugin, runtime::dx12_plugin};

fn main() {
    App::new()
        .add_plugins(dx12_plugin())
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}
