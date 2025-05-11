mod game_over_menu;
mod hud;
mod pause_menu;
mod styles;

use bevy::prelude::*;

// use hud::HudPlugin;
use game_over_menu::GameOverMenuPlugin;
use pause_menu::PauseMenuPlugin;
use hud::HUDPlugin;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HUDPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(GameOverMenuPlugin);
    }
}
