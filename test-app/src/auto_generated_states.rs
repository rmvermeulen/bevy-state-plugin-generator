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
    pub enum GameState {
        #[default]
        Loading,
        Ready,
        Exiting,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Loading)]
    pub enum GameStateLoading {
        #[default]
        Configs,
        Assets,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Loading = Loading::Configs)]
    pub struct LoadingConfigs;

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Loading = Loading::Assets)]
    pub struct LoadingAssets;

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Ready)]
    pub enum GameStateReady {
        #[default]
        Playing,
        Paused,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Ready = Ready::Playing)]
    pub struct ReadyPlaying;

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Ready = Ready::Paused)]
    pub struct ReadyPaused;

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Exiting)]
    pub struct GameStateExiting;
}
pub struct GeneratedStatesPlugin;
impl bevy::app::Plugin for GeneratedStatesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<states::GameState>()
            .add_sub_state::<states::GameStateLoading>()
            .add_sub_state::<states::LoadingConfigs>()
            .add_sub_state::<states::LoadingAssets>()
            .add_sub_state::<states::GameStateReady>()
            .add_sub_state::<states::ReadyPlaying>()
            .add_sub_state::<states::ReadyPaused>()
            .add_sub_state::<states::GameStateExiting>();
    }
}
