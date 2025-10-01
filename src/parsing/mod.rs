mod parsers;
#[cfg(test)]
mod tests;
mod tokens;

/// used by doctest
#[allow(unused_imports)]
pub use parsers::comment;
pub use parsers::{config_is_valid, parse_node, parse_states_text};
pub(crate) use tokens::*;

// TODO: investigate "tree-less parsing", see https://www.youtube.com/watch?v=NxiKlnUtyio
