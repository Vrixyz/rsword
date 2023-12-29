mod game;
pub mod word_table;
pub mod word_tree;

use bevy::prelude::*;
use game::GamePlugin;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    /*
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_line_number(true)
        .with_target(true)
        .pretty();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
    */
    App::new().add_plugins(GamePlugin).run();
}
