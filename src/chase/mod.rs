//use avian2d::prelude::*;
//use bevy::prelude::*;

mod components;
pub mod events;
mod observers;
mod plugin;
mod states;
mod systems;

pub(super) use events::{EndChaseGame, StartChaseGame};
pub(super) use plugin::ChasePlugin;
