use crate::chase::{components::Elf, states::ChaseGameState};
use bevy::prelude::*;

pub(super) fn run_game(
    mut elf: Single<&mut Transform, With<Elf>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 200.0;

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        elf.translation += direction.normalize() * speed * time.delta_secs();
    }
}

pub(super) fn toggle_game_state(
    state: Res<State<ChaseGameState>>,
    mut next_state: ResMut<NextState<ChaseGameState>>,
) {
    match state.get() {
        ChaseGameState::On => next_state.set(ChaseGameState::Off),
        ChaseGameState::Off => next_state.set(ChaseGameState::On),
    }
}
