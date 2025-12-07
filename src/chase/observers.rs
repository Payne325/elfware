use crate::{
    chase::{
        components::{Dog, Elf},
        states::ChaseGameState,
        systems::toggle_game_state,
    },
    game_manager::{EndGame, MiniGame, StartGame},
};
use bevy::prelude::*;

pub(super) fn observe_game_start(
    event: On<StartGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    if event.event().0 != MiniGame::Chase {
        return;
    }

    commands.spawn(Elf::new_bundle(&asset_server));
    commands.spawn(Dog::new_bundle(&asset_server));
    toggle_game_state(state, next_state);
}

pub(super) fn observe_game_end(
    event: On<EndGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Dog>>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    if event.event().0 != MiniGame::Chase {
        return;
    }
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    toggle_game_state(state, next_state);
}
