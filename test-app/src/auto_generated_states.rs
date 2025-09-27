// bspg:State
// bspg: tState
// bspg:root_state    RootState
// bspg:naming_scheme Full
// bspg:
// Loading {
//     Configs
//     Assets
// }
// Ready {
//     Playing
//     Paused
// }
// Exiting

#![allow(missing_docs)]
use bevy::prelude::AppExtStates;
pub mod states {
    use bevy::prelude::StateSet;
    #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
    pub struct GameState;
}
pub struct GeneratedStatesPlugin;
impl bevy::app::Plugin for GeneratedStatesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<states::GameState>();
    }
}
