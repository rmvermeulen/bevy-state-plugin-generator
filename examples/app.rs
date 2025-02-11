use bevy::prelude::{App, AppExit, DefaultPlugins};

#[cfg(feature = "dogfood")]
mod generated_states;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "dogfood")]
    app.add_plugins(generated_states::GeneratedStatesPlugin);

    println!("App created, starting...");

    let exit = app.run();
    println!("App exited with {:?}", exit);
    exit
}
