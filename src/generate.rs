use std::rc::Rc;

use iter_tools::Itertools;

use crate::model::StateConfig;

fn generate_type(parent: Option<&str>, states: &StateConfig) -> String {
    let derives = parent
        .map(|parent_name| {
            [
                "#[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]",
                &format!(
                    "#[source({parent_name} = {parent_name}::{state_name})]",
                    state_name = states.name()
                ),
            ]
            .join("\n")
        })
        .unwrap_or("#[derive(States, Hash, Default, Debug, Clone, PartialEq, Eq)]".to_owned());
    let full_name = format!("{}{}", parent.unwrap_or_default(), states.name());
    match states {
        StateConfig::Single(_) => format!("{derives}\npub struct {full_name};"),
        StateConfig::Many(_, variants) => {
            let variants = variants.iter().map(|variant| variant.name()).join(", ");
            format!("{derives}\npub enum {full_name} {{ #[default] {variants} }}")
        }
    }
}

fn generate_all_types(states: &StateConfig) -> String {
    let own_type = generate_type(None, states);
    match states {
        StateConfig::Single(_) => own_type,
        StateConfig::Many(_, variants) => {
            let variants = variants
                .iter()
                .map(|variant| generate_type(Some(states.name()), variant))
                .join("\n");
            format!("{own_type}\n\n{variants}")
        }
    }
}

pub(crate) fn generate_states_plugin(states: Rc<StateConfig>) -> String {
    format!(
        "use bevy::prelude::{{*, States}};\npub mod states {{\nuse super::*;\n{}\n}}\n{}",
        generate_all_types(&states)
            .lines()
            .map(|line| format!("  {}", line.trim_end()))
            .join("\n"),
        r#"
pub struct GeneratedStatesPlugin;
impl bevy::app::Plugin for GeneratedStatesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<states::GameState>();
    }
}"#
    )
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_generate_states_plugin() {
        let states = StateConfig::many("GameState", [
            StateConfig::single("Loading"),
            StateConfig::many("Ready", [
                StateConfig::many("Menu", [
                    StateConfig::single("Main"),
                    StateConfig::single("Options"),
                ]),
                StateConfig::many("Game", [
                    StateConfig::single("Playing"),
                    StateConfig::single("Paused"),
                    StateConfig::single("GameOver"),
                ]),
            ]),
        ]);
        assert_snapshot!(generate_states_plugin(Rc::new(states)), @r"
        use bevy::prelude::{*, States};
        pub mod states {
        use super::*;
          #[derive(States, Hash, Default, Debug, Clone, PartialEq, Eq)]
          pub enum GameState { #[default] Loading, Ready }
          
          #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
          #[source(GameState = GameState::Loading)]
          pub struct GameStateLoading;
          #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
          #[source(GameState = GameState::Ready)]
          pub enum GameStateReady { #[default] Menu, Game }
        }

        pub struct GeneratedStatesPlugin;
        impl bevy::app::Plugin for GeneratedStatesPlugin {
            fn build(&self, app: &mut bevy::app::App) {
                app.init_state::<states::GameState>();
            }
        }
        ");
    }
}
