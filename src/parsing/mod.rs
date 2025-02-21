mod parsers;
#[cfg(test)]
mod tests;

/// used by doctest
#[cfg(feature = "comments")]
pub use parsers::comment;

pub use parsers::{config_is_valid, parse_node, parse_states_file};
