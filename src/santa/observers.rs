use crate::{
    game_manager::{EndGame, GameState, MiniGame, MyMusic, StartGame, toggle_game_state},
    santa::components::{Elf, Ground, Platform, Santa},
};
use bevy::prelude::*;

pub(super) fn observe_game_start(
    event: On<StartGame>,
    camera: Single<&Camera>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    if event.event().0 != MiniGame::Santa {
        return;
    }
    let vp_size = camera.logical_viewport_size().unwrap();
    let screen_size = (vp_size.x / 2.0, vp_size.y / 2.0);

    commands.spawn(Elf::new_bundle(&asset_server, screen_size));
    commands.spawn(Santa::new_bundle(&asset_server, screen_size));
    commands.spawn(Ground::new_bundle(&asset_server, screen_size));

    for platform in Platform::new_bundles(&asset_server, screen_size) {
        commands.spawn(platform);
    }

    // commands.spawn(MyMusic::new_bundle_once_and_cleanup(&asset_server, "TODO"));

    toggle_game_state(state, next_state, MiniGame::Santa);
}

pub(super) fn observe_game_end(
    event: On<EndGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Santa>>,
    ground: Single<Entity, With<Ground>>,
    platform: Query<Entity, With<Platform>>,
    state: Res<State<GameState>>,
    next_state: ResMut<NextState<GameState>>,
) {
    if event.event().0 != MiniGame::Santa {
        return;
    }
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    commands.entity(ground.entity()).despawn();

    for p in platform.iter() {
        commands.entity(p.entity()).despawn();
    }
    toggle_game_state(state, next_state, MiniGame::Santa);
}
