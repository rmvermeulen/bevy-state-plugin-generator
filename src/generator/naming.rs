use crate::NamingScheme;
use crate::models::{ParentState, StateNode};
use crate::split_case::normalize_state_name;

pub trait ToStateName {
    fn to_state_name(&self) -> String;
}

impl<S: ToString> ToStateName for S {
    fn to_state_name(&self) -> String {
        normalize_state_name(&format!("{}State", self.to_string()))
    }
}

pub fn apply_naming_scheme(
    naming_scheme: NamingScheme,
    node: &StateNode,
    parent: Option<&ParentState>,
) -> String {
    let name = node.name().to_state_name();
    match naming_scheme {
        NamingScheme::None => name,
        NamingScheme::Short => {
            if let Some(parent) = parent {
                let parent = parent.state_name();
                let parent = parent
                    .strip_suffix("State")
                    .map(Into::into)
                    .unwrap_or(parent);
                format!("{parent}{name}").to_state_name()
            } else {
                name
            }
        }
        NamingScheme::Full => {
            todo!("track full name through parent_state")
        }
    }
}
