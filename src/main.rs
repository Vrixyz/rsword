mod game;
pub mod word_table;
pub mod word_tree;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
