pub(crate) mod core;
#[cfg(test)]
mod tests;

use std::path::Path;
use std::{fs, io};

use itertools::{Itertools, concat};

use crate::generate::core::generate_state_plugin_source;
use crate::parsing::header::parse_template_header;
use crate::prelude::PluginConfig;
use crate::processing::ProcessingError;

/// The kinds of errors that can occur
#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    /// A fs-related error occurred
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),
    /// The content is wrong
    #[error("Processing Error: {0}")]
    Processing(#[from] ProcessingError),
}

/// ```rust no_run
/// use bevy_state_plugin_generator::prelude::*;
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
) -> Result<(), GeneratorError> {
    let src_display = template_path.as_ref().to_string_lossy();
    println!("cargo:rerun-if-changed={src_display}");
    let source = std::fs::read_to_string(&template_path)?;
    let header = parse_template_header(&source, &mut plugin_config);

    let plugin_source =
        generate_state_plugin_source(&header.template.join("\n"), plugin_config, None)?;

    let header = concat([
        header
            .info_block
            .into_iter()
            .map(|line| format!("// {line}"))
            .collect_vec(),
        // vec![
        //     format!("comment_block={comment_block:?}"),
        //     format!("info_and_warnings={info_and_warnings:?}"),
        // ],
        header
            .comments_block
            .into_iter()
            .map(String::from)
            .collect_vec(),
    ])
    .join("\n");

    fs::write(&template_path, format!("{header}\n\n{plugin_source}")).map_err(Into::into)
}

/// ```rust no_run
/// use bevy_state_plugin_generator::prelude::*;
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
) -> Result<(), GeneratorError> {
    let src_display = src.as_ref().to_string_lossy();
    println!("cargo:rerun-if-changed={src_display}");
    let source = std::fs::read_to_string(&src)?;
    let source = generate_state_plugin_source(&source, plugin_config, Some(&src_display))?;
    fs::write(dst, source).map_err(Into::into)
}
