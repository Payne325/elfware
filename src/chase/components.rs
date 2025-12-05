use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub(super) struct Elf {}

#[derive(Bundle)]
pub(super) struct ElfBundle {
    elf: Elf,
    transform: Transform,
    sprite: Sprite,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
}

impl Elf {
    pub(super) fn new_bundle(asset_server: &Res<AssetServer>) -> ElfBundle {
        ElfBundle {
            elf: Elf {},
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::from_image(asset_server.load("sprites/elf.png")),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(64., 64.),
            gravity_scale: GravityScale(0.0),
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
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
}

impl Dog {
    pub(super) fn new_bundle(asset_server: &Res<AssetServer>) -> DogBundle {
        DogBundle {
            dog: Dog {},
            transform: Transform::from_xyz(-400.0, 400.0, 0.0),
            sprite: Sprite::from_image(asset_server.load("sprites/zeus_1.png")),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(109., 133.),
            gravity_scale: GravityScale(0.0),
        }
    }
}
