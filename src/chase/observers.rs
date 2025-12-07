use crate::chase::{
    components::{Dog, Elf},
    events::{EndGame, StartGame},
    states::ChaseGameState,
    systems::toggle_game_state,
};
use bevy::prelude::*;

pub(super) fn observe_game_start(
    _event: On<StartGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.spawn(Elf::new_bundle(&asset_server));
    commands.spawn(Dog::new_bundle(&asset_server));
    toggle_game_state(state, next_state);
}

pub(super) fn observe_game_end(
    _event: On<EndGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Dog>>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    toggle_game_state(state, next_state);
}
