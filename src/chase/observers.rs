use bevy::prelude::*;

use crate::chase::{
    components::Elf,
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
    commands.spawn((
        Elf {},
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(asset_server.load("sprites/elf.png")),
        //RigidBody::Dynamic,
        //add collider here
    ));
    println!("BADABING!");
    toggle_game_state(state, next_state);
}

pub(super) fn observe_game_end(
    _event: On<EndChaseGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.entity(elf.entity()).despawn();
    println!("BADABONG!");
    toggle_game_state(state, next_state);
}
