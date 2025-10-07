#![feature(trim_prefix_suffix)]
#![feature(coverage_attribute)]
#![warn(missing_docs)]
#![doc = include_str!("../Readme.md")]

pub(crate) mod config;
pub(crate) mod generate;
pub(crate) mod parsing;
pub(crate) mod processing;
#[cfg(test)]
pub(crate) mod testing;

/// The types and functions required to use this library
pub mod prelude {
    pub use crate::config::{NamingScheme, PluginConfig, PluginName};
    pub use crate::generate::{generate_plugin, update_template};
}
