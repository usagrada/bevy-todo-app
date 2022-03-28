use bevy::prelude::*;
use task_todo_app::MainPlugin;
fn main() {
    #[cfg(debug_assertions)]
    {
        // use std::fs;
        // fs::create_dir("build").unwrap();
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}
