# State-Plugin generator

Generates a bunch of hierarchical states (structs and enums, using `State`
and `SubState`) from a simple DSL and sets up their relationships
in a `Plugin`.

## Probably do not use

I made this without checking if and/or how it would be a good idea.

## versions

| bevy   | this thing |
| ------ | ---------- |
| 0.15.0 | 1.2.0      |
| 0.16.0 | 1.3.0      |
| 0.17.0 | 1.4.0      |

## usage

### single-file mode

Create a `states.rs` in `src/` and add the following comment at the top:

```rs
// bspg:
// Loading
// Ready { Menu Game }
// Exiting
```

Set up your `build.rs` like this:

```rust no_run
use bevy_state_plugin_generator::prelude::*;
fn main() {
  /// The [Default::default] configuration is:
  let config = PluginConfig {
    plugin_name: PluginName::Struct("GeneratedStatesPlugin"),
    root_state_name: Some("GameState".to_string()),
    states_module_name: "states".to_string(),
    naming_scheme: NamingScheme::Full,
    additional_derives: vec![],
  };
  update_template("src/states.rs", config)
    .expect("Failed to update template!");
}
```

### separate-file mode

Create a `states.txt` in `src/`:

```txt
// the root is implicit
Loading // comments go until the end of the line
Ready { Menu Game }                     // enum state
Exiting                                 // singleton
```

Set up your `build.rs` like this:

```rust no_run
use bevy_state_plugin_generator::prelude::*;
fn main() {
  /// The [Default::default] configuration is:
  let config = PluginConfig {
    plugin_name: PluginName::Struct("GeneratedStatesPlugin"),
    root_state_name: Some("GameState".to_string()),
    states_module_name: "states".to_string(),
    naming_scheme: NamingScheme::Full,
    additional_derives: vec![],
  };
  generate_plugin("src/states.txt", "src/generated_states.rs", config)
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

## naming

Consider the following sample:

```txt
PlayerState {
    Good
    BadState { OnFire InWater }
}
```

| none          | full                         | merge                   |
| ------------- | ---------------------------- | ----------------------- |
| `PlayerState` | `PlayerState`                | `PlayerState`           |
| `Good`        | `PlayerStateGood`            | `PlayerGoodState`       |
| `BadState`    | `PlayerStateBadState`        | `PlayerBadState`        |
| `OnFire`      | `PlayerStateBadStateOnFire`  | `PlayerBadOnFireState`  |
| `InWater`     | `PlayerStateBadStateInWater` | `PlayerBadInWaterState` |
