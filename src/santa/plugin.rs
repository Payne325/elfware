use bevy::prelude::*;

use crate::santa::{observers, states::SantaGameState, systems};

pub(crate) struct SantaPlugin;

impl Plugin for SantaPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observers::observe_game_start);
        app.add_observer(observers::observe_game_end);
        app.add_systems(
            Update,
            (
                systems::move_elf.run_if(in_state(SantaGameState::On)),
                systems::detect_santa_elf_collision.run_if(in_state(SantaGameState::On)),
            ),
        );
        app.init_state::<SantaGameState>();
    }
}
