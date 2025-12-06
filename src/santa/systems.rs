use crate::santa::{
    components::{Elf, Ground, Santa},
    states::SantaGameState,
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
    }
}

pub(super) fn toggle_game_state(
    state: Res<State<SantaGameState>>,
    mut next_state: ResMut<NextState<SantaGameState>>,
) {
    match state.get() {
        SantaGameState::On => next_state.set(SantaGameState::Off),
        SantaGameState::Off => next_state.set(SantaGameState::On),
    }
}

pub(super) fn print_started_collisions(
    mut collision_reader: MessageReader<CollisionStart>,
    mut elf: Single<(Entity, &mut Elf)>,
    ground: Single<Entity, With<Ground>>,
) {
    if elf.1.is_grounded == true {
        return;
    }

    for event in collision_reader.read() {
        if (event.collider1 == ground.entity() && event.collider2 == elf.0.entity())
            || (event.collider1 == elf.0.entity() && event.collider2 == ground.entity())
        {
            println!("ground and elf collided");
            elf.1.is_grounded = true;
            return;
        }
    }
}
