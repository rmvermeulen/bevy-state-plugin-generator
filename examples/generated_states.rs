// src: examples/states.txt
// Loading [ 
//     Config
//     Assets
// ]
// Ready {
//     Menu
//     Game {
//         Playing [
//             Player { Alive Dead }
//             Environment { Normal Danger }
//         ]
//         Paused
//         Over
//     }
// }
// [List("Loading", [Single("Config"), Single("Assets")]), Enum("Ready", [Single("Menu"), Enum("Game", [List("Playing", [Enum("Player", [Single("Alive"), Single("Dead")]), Enum("Environment", [Single("Normal"), Single("Danger")])]), Single("Paused"), Single("Over")])])]

#![allow(missing_docs)]
use bevy::prelude::AppExtStates;

pub mod states {
    use bevy::prelude::StateSet;
    #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
    pub enum GameState {
        #[default]
        Loading,
        Ready,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Loading)]
    pub struct GameStateLoading;

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Loading = Loading::Config)]
    pub struct LoadingConfig;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Loading = Loading::Assets)]
    pub struct LoadingAssets;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Ready)]
    pub enum GameStateReady {
        #[default]
        Menu,
        Game,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Ready = Ready::Menu)]
    pub struct ReadyMenu;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Ready = Ready::Game)]
    pub enum ReadyGame {
        #[default]
        Playing,
        Paused,
        Over,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Game = Game::Playing)]
    pub struct GamePlaying;

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Playing = Playing::Player)]
    pub enum PlayingPlayer {
        #[default]
        Alive,
        Dead,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Player = Player::Alive)]
    pub struct PlayerAlive;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Player = Player::Dead)]
    pub struct PlayerDead;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Playing = Playing::Environment)]
    pub enum PlayingEnvironment {
        #[default]
        Normal,
        Danger,
    }

    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Environment = Environment::Normal)]
    pub struct EnvironmentNormal;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Environment = Environment::Danger)]
    pub struct EnvironmentDanger;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Game = Game::Paused)]
    pub struct GamePaused;
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(Game = Game::Over)]
    pub struct GameOver;
}
pub struct GeneratedStatesPlugin;
impl bevy::app::Plugin for GeneratedStatesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<states::GameState>();
    }
}