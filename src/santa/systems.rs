use crate::{
    game_manager::MyMusic,
    santa::components::{Elf, Santa},
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn move_elf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut elf: Single<(&mut LinearVelocity, &mut Elf)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let elf_speed = 200.0;
    let elf_jump_speed = 500.;

    if keyboard.pressed(KeyCode::KeyW) && elf.0.y == 0.0 {
        elf.0.y = elf_jump_speed;
        commands.spawn(MyMusic::new_bundle_once(&asset_server, "audio/jump.wav"));
    }
    if keyboard.pressed(KeyCode::KeyA) {
        elf.0.x = -elf_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        elf.0.x = elf_speed;
    }
}

pub(super) fn detect_santa_elf_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_reader: MessageReader<CollisionStart>,
    elf: Single<(Entity, &Elf)>,
    santa: Single<(Entity, &Santa)>,
) {
    for event in collision_reader.read() {
        if (event.collider1 == santa.0 && event.collider2 == elf.0)
            || (event.collider1 == elf.0 && event.collider2 == santa.0)
        {
            commands.spawn(MyMusic::new_bundle_once(&asset_server, "audio/hit.wav"));
            return;
        }
    }
}
