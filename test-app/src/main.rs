cfg_if::cfg_if! {
    if #[cfg(feature = "inline")] {
        #[allow(dead_code)]
        mod auto_generated_states;
        use auto_generated_states::MyCustomStatesPlugin as GeneratedStatesPlugin;
    } else if #[cfg(feature = "full")] {
        #[allow(dead_code)]
        mod generated_states_full;
        use generated_states_full::GeneratedStatesPlugin;
    } else if #[cfg(feature = "short")] {
        #[allow(dead_code)]
        mod generated_states_short;
        use generated_states_short::GeneratedStatesPlugin;
    } else {
        #[allow(dead_code)]
        mod generated_states_none;
        use generated_states_none::GeneratedStatesPlugin;
    }
}

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((StatesPlugin, GeneratedStatesPlugin));

    app.run()
}
