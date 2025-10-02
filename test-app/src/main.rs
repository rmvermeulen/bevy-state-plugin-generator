cfg_if::cfg_if! {
    if #[cfg(feature = "inline")] {
        mod auto_generated_states;
        use auto_generated_states as generated_states;
    } else if #[cfg(feature = "full")] {
        mod generated_states_full;
        use generated_states_full as generated_states;
    } else if #[cfg(feature = "short")] {
        mod generated_states_short;
        use generated_states_short as generated_states;
    } else {
        mod generated_states_none;
        use generated_states_none as generated_states;
    }
}

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, generated_states::GeneratedStatesPlugin));

    app.run()
}
