mod game;
mod word_table;
mod word_tree;

use bevy::prelude::*;
use game::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
