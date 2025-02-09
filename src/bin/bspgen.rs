#![feature(coverage_attribute)]

use std::{env, path::PathBuf};

use bevy_state_plugin_generator::on_build_generate_plugin;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    env::args()
        .skip(1)
        .try_for_each(|path| {
            on_build_generate_plugin(
                &path,
                PathBuf::from(&path).with_extension("rs"),
                Default::default(),
            )
        })
        .expect("Failed to generate plugin(s)");
}

#[cfg(test)]
mod tests {
    use bevy_state_plugin_generator::on_build_generate_plugin;
    use rstest::rstest;
    use tempfile::NamedTempFile;

    #[rstest]
    fn the_test() {
        let input_file = NamedTempFile::new().expect("Failed to create temporary file");
        let output_file = NamedTempFile::new().expect("Failed to create temporary file");
        on_build_generate_plugin(input_file.path(), output_file.path(), Default::default())
            .expect("Failed to generate plugin(s)");
    }
}
