use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub(super) struct Elf {}

#[derive(Bundle)]
pub(super) struct ElfBundle {
    elf: Elf,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    collision_events: CollisionEventsEnabled,
    animation: AseAnimation,
    sprite: Sprite,
}

impl Elf {
    pub(super) fn new_bundle(asset_server: &Res<AssetServer>) -> ElfBundle {
        ElfBundle {
            elf: Elf {},
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(64., 64.),
            gravity_scale: GravityScale(0.0),
            collision_events: CollisionEventsEnabled,
            animation: AseAnimation {
                aseprite: asset_server.load("sprites/elf.aseprite"),
                animation: Animation::tag("run").with_repeat(AnimationRepeat::Loop),
            },
            sprite: Sprite::default(),
        }
    }
}

#[derive(Component)]
pub(super) struct Dog {}

#[derive(Bundle)]
pub(super) struct DogBundle {
    dog: Dog,
    transform: Transform,
    // sprite: Sprite,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    collision_events: CollisionEventsEnabled,
    animation: AseAnimation,
    sprite: Sprite,
}

impl Dog {
    pub(super) fn new_bundle(asset_server: &Res<AssetServer>) -> DogBundle {
        DogBundle {
            dog: Dog {},
            transform: Transform::from_xyz(-400.0, 400.0, 0.0),
            //sprite: Sprite::from_image(asset_server.load("sprites/zeus_1.png")),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(109., 133.),
            gravity_scale: GravityScale(0.0),
            collision_events: CollisionEventsEnabled,
            animation: AseAnimation {
                aseprite: asset_server.load("sprites/alexsDog.aseprite"),
                animation: Animation::tag("run").with_repeat(AnimationRepeat::Loop),
            },
            sprite: Sprite::default(),
        }
    }
}
