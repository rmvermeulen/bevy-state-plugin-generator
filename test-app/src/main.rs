mod generated_states;

use bevy_app::{App, AppExit};
use generated_states::GeneratedStatesPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(GeneratedStatesPlugin);

    app.run()
}
