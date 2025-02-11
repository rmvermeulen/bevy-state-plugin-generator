use bevy::prelude::{App, AppExit, DefaultPlugins};

mod generated_states;
use generated_states::GeneratedStatesPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GeneratedStatesPlugin)
        .run()
}
