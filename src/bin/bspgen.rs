use std::{env, path::PathBuf};

use bevy_state_plugin_generator::on_build_generate_plugin;

fn main() {
    env::args()
        .skip(1)
        .try_for_each(|path| {
            on_build_generate_plugin(&path, PathBuf::from(&path).with_extension("rs"))
        })
        .expect("Failed to generate plugin(s)");
}
