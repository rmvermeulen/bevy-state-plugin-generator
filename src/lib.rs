#![feature(coverage_attribute)]
use std::{fs, io, path::Path};

use generate::generate_states_plugin;
use parse::parse_state_config;

pub(crate) mod generate;
pub(crate) mod model;
pub(crate) mod parse;

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn on_build_generate_plugin(
    src: impl AsRef<Path> + std::fmt::Display,
    dst: impl AsRef<Path>,
) -> io::Result<()> {
    println!("cargo:rerun-if-changed={src}");
    let source = std::fs::read_to_string(&src)?;
    let config = parse_state_config(&source);
    let plugin_source = generate_states_plugin(config);

    let source_insert: String = source
        .lines()
        .fold(format!("// src: {src}\n"), |lines, line| {
            format!("{lines}// {line}\n")
        });

    fs::write(dst, [source_insert, plugin_source].join("\n"))
}
