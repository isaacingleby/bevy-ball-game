use bevy::prelude::App;
use bevy_ball_game::{
    enemy::EnemyPlugin, player::PlayerPlugin, plugins_::*, score::ScorePlugin, star::StarPlugin,
};

fn main() {
    App::new()
        .add_plugins(dx12_plugin())
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(CustomPlugin)
        .run();
}
