use bevy::prelude::*;

use crate::santa::{
    components::{Elf, Ground, Platform, Santa},
    events::{EndGame, StartGame},
    states::SantaGameState,
    systems::toggle_game_state,
};

pub(super) fn observe_game_start(
    _event: On<StartGame>,
    camera: Single<&Camera>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<SantaGameState>>,
    next_state: ResMut<NextState<SantaGameState>>,
) {
    let vp_size = camera.logical_viewport_size().unwrap();

    let width = vp_size.x / 2.0;
    let height = vp_size.y / 2.0;

    commands.spawn(Elf::new_bundle(&asset_server, (width, height)));
    commands.spawn(Santa::new_bundle(&asset_server, (width, height)));
    commands.spawn(Ground::new_bundle(&asset_server, (width, height)));

    for platform in Platform::new_bundles(&asset_server, (width, height)) {
        commands.spawn(platform);
    }

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
