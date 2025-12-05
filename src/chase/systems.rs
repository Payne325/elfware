use crate::chase::{
    components::{Dog, Elf},
    states::ChaseGameState,
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn move_elf(
    mut elf: Single<&mut LinearVelocity, With<Elf>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let elf_speed = 200.0;

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
        let vel = direction.normalize() * elf_speed;
        elf.x = vel.x;
        elf.y = vel.y;
    } else {
        elf.x = 0.0;
        elf.y = 0.0;
    }
}

pub(super) fn move_dog(
    mut dog: Single<(&mut LinearVelocity, &Transform), With<Dog>>,
    elf: Single<&Transform, (With<Elf>, Without<Dog>)>,
) {
    let dog_speed = 200.0;
    let dir = (elf.translation - dog.1.translation).normalize() * dog_speed;

    dog.0.x = dir.x;
    dog.0.y = dir.y;
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
