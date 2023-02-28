use bevy::log::LogPlugin;
use bevy::prelude::*;
use rust_game::GameServer;

fn main() {
    App::new().add_plugin(GameServer).run();
}