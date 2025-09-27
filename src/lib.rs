#![feature(coverage_attribute)]
#![warn(missing_docs)]
#![doc = include_str!("../Readme.md")]

use std::path::Path;
use std::{fs, io};

pub use config::{NamingScheme, PluginConfig, PluginName};
use generator::generate_state_plugin_source;
use itertools::Itertools;
use lazy_regex::regex;
pub use parsing::{comment, config_is_valid};

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
///   update_template(
///     "src/generated_states.rs",
///     PluginConfig::default()
///   ).expect("Failed to generate plugin!");
/// }
/// ```
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn update_template(
    template_path: impl AsRef<Path>,
    mut plugin_config: PluginConfig,
) -> io::Result<()> {
    let src_display = template_path.as_ref().to_string_lossy();
    println!("cargo:rerun-if-changed={src_display}");
    let source = std::fs::read_to_string(&template_path)?;

    let comment_block = &source
        .lines()
        .take_while(|line| line.starts_with("//"))
        .collect_vec();

    let processed_input = {
        let mut template_src = Vec::new();
        let mut in_template = false;
        for line in comment_block {
            if in_template {
                if let Some(line) = line.strip_prefix("//") {
                    template_src.push(line.trim());
                } else {
                    break;
                }
            } else if let Some(captures) =
                regex!(r#"^\s*//\s*bspg:(\w+)\s+(\w+)\s*$"#).captures(line)
            {
                let (_, [name, value]) = captures.extract();
                match name {
                    "root_state_name" => {
                        plugin_config.root_state_name = Some(value.to_string().into());
                    }
                    _ => {
                        todo!("name: {name:?} value: {value:?}");
                    }
                }
            } else if regex!(r#"^\s*//\s*bspg:\s*$"#).is_match(line) {
                in_template = true;
            } else {
                break;
            }
        }
        template_src.join("\n")
    };

    let plugin_source = match generate_state_plugin_source(&processed_input, plugin_config, None) {
        Ok(source) => source,
        Err(message) => message,
    };

    // TODO: also include `get_package_info()`
    let comment_block = comment_block.join("\n");
    fs::write(
        &template_path,
        format!("{comment_block}\n\n{plugin_source}"),
    )
}

/// ```rust no_run
/// use bevy_state_plugin_generator::*;
/// fn main() {
///   generate_plugin(
///     "src/states.txt",
///     "src/generated_states.rs",
///     PluginConfig::default()
///   ).expect("Failed to generate plugin!");
/// }
/// ```
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn generate_plugin(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    plugin_config: PluginConfig,
) -> io::Result<()> {
    let src_display = src.as_ref().to_string_lossy();
    println!("cargo:rerun-if-changed={src_display}");
    let source = std::fs::read_to_string(&src)?;
    let source = match generate_state_plugin_source(&source, plugin_config, Some(&src_display)) {
        Ok(source) => source,
        Err(message) => message,
    };
    fs::write(dst, source)
}
