#![feature(coverage_attribute)]

use std::{fs, io, path::Path};

pub use generate::PluginConfig;
use generate::generate_states_plugin;
use parse::parse_state_config;

pub(crate) mod generate;
pub(crate) mod model;
pub(crate) mod parse;

#[doc = include_str!("../Readme.md")]
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn on_build_generate_plugin(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    plugin_config: PluginConfig,
) -> io::Result<()> {
    let src_display = src.as_ref().to_string_lossy();
    println!("cargo:rerun-if-changed={src_display}");
    let source = std::fs::read_to_string(&src)?;
    let state_config = parse_state_config(&source);
    let plugin_source = generate_states_plugin(state_config, plugin_config);

    let source_insert: String = source
        .lines()
        .fold(format!("// src: {src_display}\n"), |lines, line| {
            format!("{lines}// {line}\n")
        });

    fs::write(dst, [source_insert, plugin_source].join("\n"))
}
