#![feature(coverage_attribute)]

use std::{env, path::PathBuf};

use bevy_state_plugin_generator::on_build_generate_plugin;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    env::args()
        .skip(1)
        .try_for_each(|path| {
            on_build_generate_plugin(&path, PathBuf::from(&path).with_extension("rs"), None)
        })
        .expect("Failed to generate plugin(s)");
}

#[cfg(test)]
#[test]
fn the_test() {
    use std::io::Write;

    use tempfile::NamedTempFile;

    let mut input_file = NamedTempFile::new().expect("Failed to create temporary file");
    input_file
        .write_all("FooBar".as_bytes())
        .expect("Failed to write to temporary file");

    let output_file = NamedTempFile::new().expect("Failed to create temporary file");
    on_build_generate_plugin(input_file.path(), output_file.path(), None)
        .expect("Failed to generate plugin(s)");
}
