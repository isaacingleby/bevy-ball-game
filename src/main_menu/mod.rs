use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu);
        //     .add_systems(Update, (handle_menu_input, exit_game));
    }
}

pub fn main_menu() {
    println!("You are in the main menu!");
}
