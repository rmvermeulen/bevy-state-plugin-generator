use crate::NamingScheme;
use crate::generator::generate::REQUIRED_DERIVES;
use crate::models::ParentState;

#[derive(Clone, Debug)]
pub(super) struct Context {
    pub(super) derives: Vec<String>,
    pub(super) naming_scheme: NamingScheme,
    pub(super) parent_state: Option<ParentState>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            parent_state: None,
            naming_scheme: NamingScheme::None,
            derives: REQUIRED_DERIVES.iter().map(ToString::to_string).collect(),
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
