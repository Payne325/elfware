use crate::santa::{
    components::{Elf, Ground, Platform, Santa},
    events::{EndGame, StartGame},
    states::SantaGameState,
    systems::toggle_game_state,
};
use bevy::prelude::*;

pub(super) fn observe_game_start(
    _event: On<StartGame>,
    camera: Single<&Camera>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<SantaGameState>>,
    next_state: ResMut<NextState<SantaGameState>>,
) {
    let vp_size = camera.logical_viewport_size().unwrap();
    let screen_size = (vp_size.x / 2.0, vp_size.y / 2.0);

    commands.spawn(Elf::new_bundle(&asset_server, screen_size));
    commands.spawn(Santa::new_bundle(&asset_server, screen_size));
    commands.spawn(Ground::new_bundle(&asset_server, screen_size));

    for platform in Platform::new_bundles(&asset_server, screen_size) {
        commands.spawn(platform);
    }
    toggle_game_state(state, next_state);
}

pub(super) fn observe_game_end(
    _event: On<EndGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    dawg: Single<Entity, With<Santa>>,
    ground: Single<Entity, With<Ground>>,
    platform: Query<Entity, With<Platform>>,
    state: Res<State<SantaGameState>>,
    next_state: ResMut<NextState<SantaGameState>>,
) {
    commands.entity(elf.entity()).despawn();
    commands.entity(dawg.entity()).despawn();
    commands.entity(ground.entity()).despawn();

    for p in platform.iter() {
        commands.entity(p.entity()).despawn();
    }
    toggle_game_state(state, next_state);
}
