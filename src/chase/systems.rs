use crate::chase::components::{Dog, Elf};
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

pub(super) fn print_started_collisions(mut collision_reader: MessageReader<CollisionStart>) {
    if collision_reader.read().count() > 0 {
        println!("dog has got the elf!");
    }
}
