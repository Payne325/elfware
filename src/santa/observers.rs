use bevy::prelude::*;

use crate::santa::{
    components::{Elf, Santa},
    events::{EndGame, StartGame},
    states::SantaGameState,
    systems::toggle_game_state,
};

pub(super) fn observe_game_start(
    _event: On<StartGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<SantaGameState>>,
    next_state: ResMut<NextState<SantaGameState>>,
) {
    commands.spawn(Elf::new_bundle(&asset_server));
    commands.spawn(Santa::new_bundle(&asset_server));

    println!("BADABING!");
    toggle_game_state(state, next_state);
}

pub(super) fn observe_game_end(
    _event: On<EndGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Santa>>,
    state: Res<State<SantaGameState>>,
    next_state: ResMut<NextState<SantaGameState>>,
) {
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    println!("BADABONG!");
    toggle_game_state(state, next_state);
}
