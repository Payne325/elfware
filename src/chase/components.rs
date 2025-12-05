use bevy::prelude::*;

#[derive(Component)]
pub(super) struct Elf {}

#[derive(Bundle)]
pub(super) struct ElfBundle {
    elf: Elf,
    transform: Transform,
    sprite: Sprite,
    //RigidBody::Dynamic,
    //add collider here
}

impl Elf {
    pub(super) fn new_bundle(asset_server: &Res<AssetServer>) -> ElfBundle {
        ElfBundle {
            elf: Elf {},
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::from_image(asset_server.load("sprites/elf.png")),
            //RigidBody::Dynamic,
            //add collider here
        }
    }
}

#[derive(Component)]
pub(super) struct Dog {}

#[derive(Bundle)]
pub(super) struct DogBundle {
    dog: Dog,
    transform: Transform,
    sprite: Sprite,
    //RigidBody::Dynamic,
    //add collider here
}

impl Dog {
    pub(super) fn new_bundle(asset_server: &Res<AssetServer>) -> DogBundle {
        DogBundle {
            dog: Dog {},
            transform: Transform::from_xyz(-400.0, 400.0, 0.0),
            sprite: Sprite::from_image(asset_server.load("sprites/zeus_1.png")),
            //RigidBody::Dynamic,
            //add collider here
        }
    }
}
