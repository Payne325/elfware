mod components;
mod events;
mod plugin;
mod systems;

use bevy::prelude::*;
use strum_macros::EnumIter;

pub(crate) use components::MyMusic;
pub(crate) use events::{EndGame, StartGame};
pub(crate) use plugin::GameManagerPlugin;
pub(crate) use systems::toggle_game_state;

// All new games need an entry here.
#[derive(Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub(crate) enum MiniGame {
    Chase,
    Santa,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    On(MiniGame),
    #[default]
    Off,
}
