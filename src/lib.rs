#![feature(coverage_attribute)]
#![warn(missing_docs)]
#![doc = include_str!("../Readme.md")]

use std::{fs, io, path::Path};

use generator::generate_state_plugin_source;

pub use config::{NamingScheme, PluginConfig};
#[cfg(feature = "comments")]
pub use parsing::comment;
pub use parsing::config_is_valid;

/// config structs
pub mod config;
pub(crate) mod generator;
pub(crate) mod models;
pub(crate) mod parsing;
#[cfg(test)]
pub(crate) mod testing;
pub(crate) mod tokens;

/// ```rust no_run
/// use bevy_state_plugin_generator::*;
/// fn main() {
///   on_build_generate_plugin("src/states.txt", "src/generated_states.rs", PluginConfig::default())
///     .expect("Failed to generate plugin!");
/// }
/// ```
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn on_build_generate_plugin(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    plugin_config: PluginConfig,
) -> io::Result<()> {
    let src_display = src.as_ref().to_string_lossy();
    println!("cargo:rerun-if-changed={src_display}");
    let source = std::fs::read_to_string(&src)?;
    let source = match generate_state_plugin_source(src_display, source, plugin_config) {
        Ok(source) => source,
        Err(message) => message,
    };
    fs::write(dst, source)
}
