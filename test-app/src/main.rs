cfg_if::cfg_if! {
    if #[cfg(feature = "inline")] {
        #[allow(dead_code)]
        mod auto_generated_states;
        use auto_generated_states as generated_states;
    } else if #[cfg(feature = "full")] {
        #[allow(dead_code)]
        mod generated_states_full;
        use generated_states_full as generated_states;
    } else if #[cfg(feature = "short")] {
        #[allow(dead_code)]
        mod generated_states_short;
        use generated_states_short as generated_states;
    } else {
        #[allow(dead_code)]
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
