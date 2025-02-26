# State-Plugin generator

Generates a bunch of hierarchical states (structs and enums, using `State`
and `SubState`) and sets up their relationships
in a `Plugin`.

## versions

| bevy   | this thing |
| ------ | ---------- |
| 0.15.2 | 1.2.0      |

## bugs

```rust
// TODO: fix naming-scheme collision detection
// TODO: allow customizing derived traits
// TODO: allow specifying naming-scheme in states.txt per-node
```

## usage

Create a `states.txt` in `src/`:

```txt
// the root is implicit
Loading // comments go until the end of the line
Ready { Menu Game }                     // enum state
Exiting                                 // singleton
```

```rust
# // this is just here to check the above example
# use bevy_state_plugin_generator::config_is_valid;
# assert!(config_is_valid("Read { Config Assets }"));
# #[cfg(feature = "lists")]
# assert!(config_is_valid("Loading [ Config Assets ]"));
# #[cfg(feature = "comments")]
# assert!(config_is_valid("Loading // Comments"));
```

Set up your `build.rs` like this:

```rust no_run
use bevy_state_plugin_generator::*;
fn main() {
  /// The [Default::default] configuration is:
  let config = PluginConfig {
    plugin_name: "GeneratedStatesPlugin",
    state_name: "GameState",
    states_module_name: "states",
    scheme: NamingScheme::Full,
    additional_derives: &[],
  };
  on_build_generate_plugin("src/states.txt", "src/generated_states.rs", config)
    .expect("Failed to generate plugin!");
}
```

And it will generate something like the following:

```rust no_run
use bevy::prelude::{App, AppExtStates};
pub mod states {
    use bevy::prelude::{States, SubStates, StateSet};
    #[derive(States, Hash, Default, Debug, Clone, PartialEq, Eq)]
    pub enum GameState { #[default] Loading, Ready, Exiting }
    #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Loading)]
    pub struct GameStateLoading;
    #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameStateLoading = GameStateLoading)]
    pub struct GameStateLoadingConfigs;
    #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameStateLoading = GameStateLoading)]
    pub struct GameStateLoadingAssets;
    #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Ready)]
    pub enum GameStateReady { #[default] Menu, Game }
}
pub struct GeneratedStatesPlugin;
impl bevy::app::Plugin for GeneratedStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<states::GameState>();
    }
}
```
