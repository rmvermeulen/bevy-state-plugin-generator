use std::borrow::Cow;
use std::ops;

#[cfg(test)]
use bevy_reflect::Reflect;
use itertools::Itertools;

/// How state-names are determined
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Reflect))]
pub enum NamingScheme {
    /// Name includes the names of all ancestors
    Full,
    /// Name includes only the name of the immediate parent
    Short,
    /// None (all names must be unique)
    None,
}

impl NamingScheme {
    /// Get the name of the naming scheme
    /// ```rust
    /// # use bevy_state_plugin_generator::prelude::NamingScheme;
    /// assert_eq!(NamingScheme::Full.name(), "Full");
    /// assert_eq!(NamingScheme::Short.name(), "Short");
    /// assert_eq!(NamingScheme::None.name(), "None");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            NamingScheme::Full => "Full",
            NamingScheme::Short => "Short",
            NamingScheme::None => "None",
        }
    }
    /// Get the identifying tag for this scheme
    /// ```rust
    /// # use bevy_state_plugin_generator::prelude::NamingScheme;
    /// assert_eq!(NamingScheme::Full.tag(), "full");
    /// assert_eq!(NamingScheme::Short.tag(), "short");
    /// assert_eq!(NamingScheme::None.tag(), "none");
    /// ```
    pub fn tag(&self) -> String {
        self.name().to_lowercase()
    }
    /// Convert a string into a NamingScheme, if it's the name or tag
    pub fn try_parse(input: &str) -> Option<Self> {
        match input {
            "full" | "Full" => Some(NamingScheme::Full),
            "short" | "Short" => Some(NamingScheme::Short),
            "none" | "None" => Some(NamingScheme::None),
            _ => None,
        }
    }
}

impl Default for NamingScheme {
    fn default() -> Self {
        Self::Full
    }
}

impl std::fmt::Display for NamingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// How the plugin is rendered.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Reflect))]
pub enum PluginName<'s> {
    /// ```rust
    /// use bevy_state_plugin_generator::prelude::PluginName;
    /// PluginName::new_struct("MyPlugin");
    /// ```
    Struct(Cow<'s, str>),
    /// ```rust
    /// use bevy_state_plugin_generator::prelude::PluginName;
    /// PluginName::new_function("my_plugin");
    /// ```
    Function(Cow<'s, str>),
}

impl<'s> PluginName<'s> {
    /// Create a the appropriate variant
    pub fn parse<S: Into<Cow<'s, str>>>(input: S) -> Option<Self> {
        Self::try_from(input.into()).ok()
    }
    /// Create the Struct variant
    pub fn new_struct<S: Into<Cow<'s, str>>>(name: S) -> Self {
        Self::Struct(name.into())
    }
    /// Create the Function variant
    pub fn new_function<S: Into<Cow<'s, str>>>(name: S) -> Self {
        Self::Function(name.into())
    }
}

impl<'a> ops::Deref for PluginName<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Struct(name) => name,
            Self::Function(name) => name,
        }
    }
}

impl<'s> TryFrom<&'s str> for PluginName<'s> {
    type Error = ();
    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        if let Some(c) = value.chars().next()
            && c.is_alphabetic()
        {
            Ok(if c.is_uppercase() {
                Self::new_struct(value)
            } else {
                Self::new_function(value)
            })
        } else {
            Err(())
        }
    }
}

impl<'s> TryFrom<String> for PluginName<'s> {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(c) = value.chars().next()
            && c.is_alphabetic()
        {
            Ok(if c.is_uppercase() {
                Self::new_struct(value)
            } else {
                Self::new_function(value)
            })
        } else {
            Err(())
        }
    }
}

impl<'s> TryFrom<Cow<'s, str>> for PluginName<'s> {
    type Error = ();
    fn try_from(value: Cow<'s, str>) -> Result<Self, Self::Error> {
        match value {
            Cow::Borrowed(s) => Self::try_from(s),
            Cow::Owned(s) => Self::try_from(s),
        }
    }
}

/// Configuration for the generated plugin
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Reflect))]
pub struct PluginConfig {
    /// Name of the struct that implements [`bevy::plugin::Plugin`]
    /// Defaults to `GeneratedStatesPlugin`
    pub plugin_name: PluginName<'static>,
    /// Name of the root enum/struct. `None` means NO root node.
    pub root_state_name: Option<Cow<'static, str>>,
    /// Name for the inner module containing the generated states
    pub states_module_name: Cow<'static, str>,
    /// How generated states are named
    pub naming_scheme: NamingScheme,
    /// These additional traits will be added to the derive list
    pub additional_derives: Vec<Cow<'static, str>>,
}

impl PluginConfig {
    const fn const_default() -> Self {
        Self {
            plugin_name: PluginName::Struct(Cow::Borrowed("GeneratedStatesPlugin")),
            root_state_name: Some(Cow::Borrowed("GameState")),
            states_module_name: Cow::Borrowed("states"),
            naming_scheme: NamingScheme::Full,
            additional_derives: vec![],
        }
    }
    /// Set the plugin name to a struct name (`UpperCamelCase`)
    pub fn with_plugin_struct_name<S: ToString>(mut self, name: S) -> Self {
        self.plugin_name = PluginName::Struct(Cow::Owned(name.to_string()));
        self
    }
    /// Set the plugin name to a function name (`snake_case`)
    pub fn with_plugin_fn_name<S: ToString>(mut self, name: S) -> Self {
        self.plugin_name = PluginName::Function(Cow::Owned(name.to_string()));
        self
    }
    /// Set the name of the inner module containing the generated states
    pub fn with_states_module_name<S: ToString>(mut self, name: S) -> Self {
        self.states_module_name = Cow::Owned(name.to_string());
        self
    }
    /// Set the name of the root enum/struct. `None` means NO root node.
    pub fn with_root_state_name<S: ToString>(mut self, name: S) -> Self {
        self.root_state_name = Some(Cow::Owned(name.to_string()));
        self
    }
    /// Configure how generated states are named
    pub fn with_naming_scheme(mut self, scheme: NamingScheme) -> Self {
        self.naming_scheme = scheme;
        self
    }
    /// Set additional traits to derive on the generated states
    pub fn with_additional_derives<S: ToString, D: IntoIterator<Item = S>>(
        mut self,
        derives: D,
    ) -> Self {
        self.additional_derives.extend_from_slice(
            &derives
                .into_iter()
                .map(|t| t.to_string())
                .map(Cow::Owned)
                .collect_vec(),
        );
        self.additional_derives = self
            .additional_derives
            .iter()
            .unique()
            .cloned()
            .collect_vec();
        self
    }
}

/// Default configuration for the generated plugin
/// ```rust
/// # use bevy_state_plugin_generator::prelude::*;
/// let config = PluginConfig::default();
/// assert_eq!(config.plugin_name, PluginName::new_struct("GeneratedStatesPlugin"));
/// assert_eq!(config.root_state_name, Some(Cow::from("GameState")));
/// assert_eq!(config.states_module_name, Cow::from("states"));
/// assert_eq!(config.naming_scheme, NamingScheme::Full);
/// ```
impl Default for PluginConfig {
    fn default() -> Self {
        Self::const_default()
    }
}

impl From<NamingScheme> for PluginConfig {
    fn from(naming_scheme: NamingScheme) -> Self {
        Self {
            naming_scheme,
            ..Default::default()
        }
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case(NamingScheme::None)]
#[case(NamingScheme::Short)]
#[case(NamingScheme::Full)]
fn test_plugin_config_from_naming_scheme(#[case] naming_scheme: NamingScheme) {
    let config = PluginConfig::from(naming_scheme);
    assert_eq!(config.naming_scheme, naming_scheme);
}
