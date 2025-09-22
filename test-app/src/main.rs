mod generated_states_full;
mod generated_states_none;
mod generated_states_short;

use bevy::prelude::{App, AppExit};
use bevy::state::app::StatesPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, generated_states_none::GeneratedStatesPlugin));

    app.run()
}
