use crate::{
    chase::components::{Dog, Elf},
    game_manager::{EndGame, GameState, MiniGame, MyMusic, StartGame, toggle_game_state},
};
use bevy::prelude::*;

pub(super) fn observe_game_start(
    event: On<StartGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    if event.event().0 != MiniGame::Chase {
        return;
    }

    commands.spawn(Elf::new_bundle(&asset_server));
    commands.spawn(Dog::new_bundle(&asset_server));
    commands.spawn(MyMusic::new_bundle_loop(
        &asset_server,
        "audio/chase_music.wav",
    ));
    toggle_game_state(state, next_state, MiniGame::Chase);
}

pub(super) fn observe_game_end(
    event: On<EndGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Dog>>,
    state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    if event.event().0 != MiniGame::Chase {
        return;
    }
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    toggle_game_state(state, next_state, MiniGame::Chase);
}
