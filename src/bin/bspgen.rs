#![feature(coverage_attribute)]

use std::{env, path::PathBuf};

use bevy_state_plugin_generator::on_build_generate_plugin;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    env::args()
        .skip(1)
        .try_for_each(|path| {
            on_build_generate_plugin(&path, PathBuf::from(&path).with_extension("rs"))
        })
        .expect("Failed to generate plugin(s)");
}
