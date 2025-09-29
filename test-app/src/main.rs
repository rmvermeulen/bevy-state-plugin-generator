#[cfg(feature = "inline")]
mod auto_generated_states;
#[cfg(feature = "full")]
mod generated_states_full;
mod generated_states_none;
#[cfg(feature = "short")]
mod generated_states_short;

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, generated_states_none::GeneratedStatesPlugin));

    app.run()
}
