use derive_more::Deref;
use derive_more::From;
use indoc::formatdoc;
use std::fmt::{self, Display};
use std::{io, rc::Rc};

use iter_tools::Itertools;

use crate::model::{SourceState, StateNode, SubTree};
use crate::parser::parse_states_file;
use crate::{NamingScheme, PluginConfig};

#[cfg(test)]
mod tests;

const DERIVES: &str = "Hash, Default, Debug, Clone, PartialEq, Eq";

#[derive(Debug, Clone)]
struct Context {
    derives: String,
    naming_scheme: NamingScheme,
    source_state: Option<SourceState>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            source_state: None,
            naming_scheme: NamingScheme::None,
            derives: DERIVES.to_string(),
        }
    }
}

impl From<SourceState> for Context {
    fn from(source_state: SourceState) -> Self {
        Self {
            source_state: Some(source_state),
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

impl From<(SourceState, NamingScheme)> for Context {
    fn from((source_state, naming_scheme): (SourceState, NamingScheme)) -> Self {
        Self {
            naming_scheme,
            source_state: Some(source_state),
            ..Default::default()
        }
    }
}

impl From<(NamingScheme, SourceState)> for Context {
    fn from((naming_scheme, source_state): (NamingScheme, SourceState)) -> Self {
        Self {
            naming_scheme,
            source_state: Some(source_state),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, From, Deref)]
struct TypeDefinitions(Vec<TypeDef>);

impl TypeDefinitions {
    fn take(self) -> Vec<TypeDef> {
        self.0
    }
}

impl Display for TypeDefinitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().join("\n\n"))
    }
}

#[derive(Debug, Clone)]
struct TypeDef {
    typename: String,
    source: String,
}

impl Display for TypeDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

fn get_typedef(
    node: &StateNode,
    Context {
        naming_scheme,
        source_state,
        derives,
    }: Context,
) -> TypeDef {
    let derives = source_state
        .clone()
        .map(|source_state| {
            let source = source_state.name();
            let variant = source_state.name_and_variant();
            [
                format!("#[derive(bevy::prelude::SubStates, {derives})]"),
                format!("#[source({source} = {variant})]"),
            ]
            .join("\n")
        })
        .unwrap_or(format!("#[derive(bevy::prelude::States, {derives})]"));
    let typename = if naming_scheme == NamingScheme::None {
        node.name().to_string()
    } else {
        source_state
            .clone()
            .map(|source_state| format!("{}{}", source_state.name(), node.name()))
            .unwrap_or_else(|| node.name().to_string())
    };
    match node {
        #[cfg(feature = "lists")]
        StateNode::List(_, _) => TypeDef {
            source: format!("{derives}\npub struct {typename};"),
            typename,
        },
        StateNode::Singleton(_) => TypeDef {
            source: format!("{derives}\npub struct {typename};"),
            typename,
        },
        StateNode::Enum(_, variants) => TypeDef {
            source: {
                let variants = variants.iter().map(|variant| variant.name()).join(", ");
                format!("{derives}\npub enum {typename} {{ #[default] {variants} }}")
            },
            typename,
        },
    }
}

fn generate_all_type_definitions(root_node: &StateNode, context: Context) -> TypeDefinitions {
    let root_typedef = get_typedef(root_node, context.clone());

    match root_node {
        StateNode::Singleton(_) => vec![root_typedef].into(),
        StateNode::Enum(_, variants) => {
            let mut variants = variants
                .iter()
                .flat_map(|child_node| {
                    let source_state = SourceState {
                        name: match context.naming_scheme {
                            NamingScheme::Short | NamingScheme::None => {
                                root_node.name().to_string()
                            }
                            NamingScheme::Full => root_typedef.typename.clone(),
                        },
                        variant: child_node.name().to_string(),
                    };
                    generate_all_type_definitions(child_node, Context {
                        source_state: Some(source_state),
                        derives: context.derives.clone(),
                        naming_scheme: context.naming_scheme,
                    })
                    .take()
                })
                .collect_vec();
            {
                let mut typedefs = vec![root_typedef];
                typedefs.append(&mut variants);
                typedefs.into()
            }
        }
        #[cfg(feature = "lists")]
        StateNode::List(_, _variants) => unimplemented!(),
        // variants
        //     .iter()
        //     .map(|child_node| {
        //         generate_all_type_definitions(
        //             Some(SourceState {
        //                 name: typename.clone(),
        //                 variant: child_node.name().to_string(),
        //             }),
        //             child_node,
        //             scheme,
        //         )
        //     })
        //     .flatten()
        //     .collect_vec(),
    }
}

pub fn generate_debug_info(src_path: &str, source: &str) -> String {
    let pkg = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let lines = source.lines().map(|line| format!("// {line}")).join("\n");
    formatdoc! {"
        // generated by {pkg} v{version}
        // src: {src_path}
        {lines}
    "}
}

pub(crate) fn generate_plugin_source(root_state: Rc<StateNode>, config: PluginConfig) -> String {
    let PluginConfig {
        plugin_name,
        state_name,
        states_module_name,
        scheme: _,
    } = config;

    let type_definitions = generate_all_type_definitions(&root_state, Context::from(config.scheme));
    formatdoc! {"
        #![allow(missing_docs)]
        use bevy::prelude::AppExtStates;
        pub mod {states_module_name} {{
            use bevy::prelude::StateSet;
            {type_definitions}
        }}
        pub struct {plugin_name};
        impl bevy::app::Plugin for {plugin_name} {{
            fn build(&self, app: &mut bevy::app::App) {{ 
                app.init_state::<{states_module_name}::{state_name}>();
            }}
        }}
    "}
}

#[cfg(feature = "rustfmt")]
pub fn try_format_source(source: &str) -> io::Result<String> {
    duct::cmd!("rustfmt")
        .stdin_bytes(source)
        .stderr_to_stdout()
        .read()
}

pub fn format_source<S: AsRef<str>>(source: S) -> String {
    let source = source.as_ref();
    #[cfg(feature = "rustfmt")]
    {
        try_format_source(source).unwrap_or_else(|_| source.to_owned())
    }
    #[cfg(not(feature = "rustfmt"))]
    {
        source.to_owned()
    }
}

pub fn generate_state_plugin_source<P: AsRef<str> + std::fmt::Display, S: AsRef<str>>(
    src_path: P,
    source: S,
    plugin_config: PluginConfig,
) -> Result<String, String> {
    let source = source.as_ref();
    let parse_tree =
        parse_states_file(source, plugin_config.state_name).map_err(|e| e.to_string())?;

    let parse_tree_size = parse_tree.get_tree_size();

    let root_node: Rc<StateNode> = parse_tree
        .try_into()
        .map(Rc::new)
        .map_err(|e| format!("{e:?}"))?;
    let state_tree_size = root_node.get_tree_size();

    if state_tree_size > parse_tree_size {
        return Err("state-tree exceeds parse-tree!".into());
    }

    let debug_info = generate_debug_info(src_path.as_ref(), source);
    let plugin_source = generate_plugin_source(root_node, plugin_config);
    let source = [debug_info, plugin_source].join("\n");

    Ok(format_source(source))
}
