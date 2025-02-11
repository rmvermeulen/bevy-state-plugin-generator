#![feature(coverage_attribute)]

use std::{fs, io, path::Path};

use generate::generate_full_source;
use parser::parse_config;

pub(crate) mod generate;
pub(crate) mod model;
pub(crate) mod nodes;
pub(crate) mod parser;
pub(crate) mod tokens;

pub use model::{NamingScheme, PluginConfig};

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
    let source = match generate_full_source(src_display, source, plugin_config) {
        Ok(source) => source,
        Err(message) => message,
    };
    fs::write(dst, source)
}
