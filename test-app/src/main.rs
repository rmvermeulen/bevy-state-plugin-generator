mod generated_states;

use bevy::prelude::{App, AppExit};
use bevy::state::app::StatesPlugin;
use generated_states::GeneratedStatesPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, GeneratedStatesPlugin));

    app.run()
}
