use crate::{
    game_manager::{GameState, MiniGame},
    santa::{observers, systems},
};
use bevy::prelude::*;

pub(crate) struct SantaPlugin;

impl Plugin for SantaPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observers::observe_game_start);
        app.add_observer(observers::observe_game_end);
        app.add_systems(
            Update,
            (
                systems::move_elf.run_if(in_state(GameState::On(MiniGame::Santa))),
                systems::detect_santa_elf_collision
                    .run_if(in_state(GameState::On(MiniGame::Santa))),
            ),
        );
    }
}
