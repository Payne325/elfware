use bevy::prelude::*;

use crate::chase::{observers, states::ChaseGameState, systems};

pub(crate) struct ChasePlugin;

impl Plugin for ChasePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observers::observe_game_start);
        app.add_observer(observers::observe_game_end);
        app.add_systems(
            Update,
            (
                systems::move_elf.run_if(in_state(ChaseGameState::On)),
                systems::move_dog.run_if(in_state(ChaseGameState::On)),
            ),
        );
        app.init_state::<ChaseGameState>();
    }
}
