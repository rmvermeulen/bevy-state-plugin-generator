use super::REQUIRED_DERIVES;
use crate::{NamingScheme, models::ParentState};

#[derive(Debug, Clone)]
pub(super) struct Context {
    pub(super) derives: String,
    pub(super) naming_scheme: NamingScheme,
    pub(super) parent_state: Option<ParentState>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            parent_state: None,
            naming_scheme: NamingScheme::None,
            derives: REQUIRED_DERIVES.join(", "),
        }
    }
}

impl From<ParentState> for Context {
    fn from(parent_state: ParentState) -> Self {
        Self {
            parent_state: Some(parent_state),
            ..Default::default()
        }
    }
}

impl From<NamingScheme> for Context {
    fn from(naming_scheme: NamingScheme) -> Self {
        Self {
            naming_scheme,
            ..Default::default()
        }
    }
}

impl From<(ParentState, NamingScheme)> for Context {
    fn from((parent_state, naming_scheme): (ParentState, NamingScheme)) -> Self {
        Self {
            naming_scheme,
            parent_state: Some(parent_state),
            ..Default::default()
        }
    }
}

impl From<(NamingScheme, ParentState)> for Context {
    fn from((naming_scheme, parent_state): (NamingScheme, ParentState)) -> Self {
        Self {
            naming_scheme,
            parent_state: Some(parent_state),
            ..Default::default()
        }
    }
}
