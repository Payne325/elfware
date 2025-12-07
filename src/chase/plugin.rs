use crate::{
    chase::{observers, systems},
    game_manager::{GameState, MiniGame},
};
use bevy::prelude::*;

pub(crate) struct ChasePlugin;

impl Plugin for ChasePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observers::observe_game_start);
        app.add_observer(observers::observe_game_end);
        app.add_systems(
            Update,
            (
                systems::move_elf.run_if(in_state(GameState::On(MiniGame::Chase))),
                systems::move_dog.run_if(in_state(GameState::On(MiniGame::Chase))),
                systems::print_started_collisions.run_if(in_state(GameState::On(MiniGame::Chase))),
            ),
        );
    }
}
