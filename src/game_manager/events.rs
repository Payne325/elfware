use crate::game_manager::MiniGame;
use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct StartGame(pub(crate) MiniGame);

#[derive(Event)]
pub(crate) struct EndGame(pub(crate) MiniGame);
