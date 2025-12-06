use crate::santa::{
    components::{Elf, Santa},
    states::SantaGameState,
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn move_elf(
    mut elf: Single<(&mut LinearVelocity, &mut Elf)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let elf_speed = 200.0;
    let elf_jump_speed = 500.;

    if keyboard.pressed(KeyCode::KeyW) && elf.0.y == 0.0 {
        elf.0.y = elf_jump_speed;
        //elf.1.is_grounded = false;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        elf.0.x = -elf_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        elf.0.x = elf_speed;
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

pub(super) fn detect_santa_elf_collision(
    mut collision_reader: MessageReader<CollisionStart>,
    elf: Single<(Entity, &Elf)>,
    santa: Single<(Entity, &Santa)>,
) {
    for event in collision_reader.read() {
        if (event.collider1 == santa.0 && event.collider2 == elf.0)
            || (event.collider1 == elf.0 && event.collider2 == santa.0)
        {
            println!("santa and elf collided");
            return;
        }
    }
}
