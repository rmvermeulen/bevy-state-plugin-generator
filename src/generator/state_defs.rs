use std::rc::Rc;

use indoc::formatdoc;
use itertools::Itertools;

use crate::NamingScheme;
use crate::generator::context::Context;
use crate::generator::models::{StateDef, StateDefinitions};
use crate::generator::naming::apply_naming_scheme;
use crate::models::{DefinedStates, ParentState, StateNode};

fn get_statedef(
    node: &StateNode,
    Context {
        naming_scheme,
        parent_state,
        derives,
    }: Context,
) -> StateDef {
    let derives = derives.iter().unique().join(", ");
    let derives = parent_state
        .clone()
        .map(|parent_state| {
            let source = parent_state.state_name();
            let variant = parent_state.name_and_variant();
            formatdoc! {"
                #[derive(bevy::prelude::SubStates, {derives})]
                #[source({source} = {variant})]
            "}
        })
        .unwrap_or_else(|| formatdoc! {"#[derive(bevy::prelude::States, {derives})]"})
        .trim()
        .to_string();
    let typename = apply_naming_scheme(naming_scheme, node, parent_state.as_ref());
    let source_for_struct = || {
        formatdoc! {"
            {derives}
            pub struct {typename};
        "}
    };
    let source_for_enum = |variants: &Vec<Rc<StateNode>>| {
        let variants = variants
            .iter()
            .map(|variant| variant.name())
            .join(",\n      ");
        formatdoc! {"
            {derives}
            pub enum {typename} {{
                #[default]
                {variants}
            }}
        "}
    };
    let parent_name = parent_state.map(|s| s.state_name());
    match node {
        StateNode::List(_, _) => StateDef {
            parent_name,
            source: source_for_struct(),
            typename,
        },
        StateNode::Singleton(_) => StateDef {
            parent_name,
            source: source_for_struct(),
            typename,
        },
        StateNode::Enum(_, variants) => StateDef {
            parent_name,
            source: if variants.is_empty() {
                source_for_struct()
            } else {
                source_for_enum(variants)
            },
            typename,
        },
    }
}

fn generate_state_definitions_rec(root_node: &StateNode, context: Context) -> StateDefinitions {
    let root_statedef = get_statedef(root_node, context.clone());
    match root_node {
        StateNode::Singleton(_) => vec![root_statedef].into(),
        StateNode::Enum(_, variants) => {
            let mut variants = variants
                .iter()
                .flat_map(|child_node| {
                    let parent_state = ParentState::new(
                        match context.naming_scheme {
                            NamingScheme::Full => root_statedef.typename.clone(),
                            NamingScheme::Short => root_node.name().to_string(),
                            NamingScheme::None => root_node.name().to_string(),
                        },
                        child_node.name(),
                        context.parent_state.clone(),
                    );
                    generate_state_definitions_rec(
                        child_node,
                        Context {
                            parent_state: Some(parent_state),
                            derives: context.derives.clone(),
                            naming_scheme: context.naming_scheme,
                        },
                    )
                    .inner()
                })
                .collect_vec();
            {
                let mut statedefs = vec![root_statedef];
                statedefs.append(&mut variants);
                statedefs.into()
            }
        }
        StateNode::List(_, variants) => {
            let mut variants = variants
                .iter()
                .flat_map({
                    |child_node| {
                        // NOTE: pass along current Context since List does not actually render
                        // into a struct, but refers to its parent
                        generate_state_definitions_rec(child_node, context.clone()).inner()
                    }
                })
                .collect_vec();
            {
                let mut typedefs = vec![root_statedef];
                typedefs.append(&mut variants);
                typedefs.into()
            }
        }
    }
}

pub(crate) fn generate_all_state_definitions(
    defined_states: DefinedStates,
    context: Context,
) -> StateDefinitions {
    match defined_states {
        DefinedStates::Unrelated(states) => states
            .iter()
            .flat_map({
                |child_node| generate_state_definitions_rec(child_node, context.clone()).inner()
            })
            .collect_vec()
            .into(),
        DefinedStates::Root(root_node) => generate_state_definitions_rec(&root_node, context),
    }
}
