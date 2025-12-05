use bevy::prelude::*;

use crate::chase::{
    components::{Dog, Elf},
    events::{EndChaseGame, StartChaseGame},
    states::ChaseGameState,
    systems::toggle_game_state,
};

pub(super) fn observe_game_start(
    _event: On<StartChaseGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.spawn(Elf::new_bundle(&asset_server));
    commands.spawn(Dog::new_bundle(&asset_server));

    println!("BADABING!");
    toggle_game_state(state, next_state);
}

pub(super) fn observe_game_end(
    _event: On<EndChaseGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Dog>>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    println!("BADABONG!");
    toggle_game_state(state, next_state);
}
