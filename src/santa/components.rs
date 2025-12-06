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
            rigid_body: RigidBody::Dynamic,
            collider: Collider::rectangle(64., 64.),
            gravity_scale: GravityScale(1.0),
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
pub(super) struct Santa {}

#[derive(Bundle)]
pub(super) struct SantaBundle {
    santa: Santa,
    transform: Transform,
    sprite: Sprite,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    collision_events: CollisionEventsEnabled,
}

impl Santa {
    pub(super) fn new_bundle(
        asset_server: &Res<AssetServer>,
        screen_size: (f32, f32),
    ) -> SantaBundle {
        // top left of screen as origin is centre
        let x_pos = screen_size.0 / 2.0;
        let y_pos = screen_size.1 / 2.0;

        println!("santa position: {:?}", (x_pos, y_pos));

        SantaBundle {
            santa: Santa {},
            transform: Transform::from_xyz(x_pos, y_pos, 0.0),
            sprite: Sprite::from_image(asset_server.load("sprites/zeus_1.png")),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(109., 133.),
            gravity_scale: GravityScale(0.0),
            collision_events: CollisionEventsEnabled,
        }
    }
}

#[derive(Component)]
pub(super) struct Ground {}

#[derive(Bundle)]
pub(super) struct GroundBundle {
    ground: Ground,
    transform: Transform,
    pub(super) sprite: Sprite,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    // collision_events: CollisionEventsEnabled,
}

impl Ground {
    pub(super) fn new_bundle(
        asset_server: &Res<AssetServer>,
        screen_size: (f32, f32),
    ) -> GroundBundle {
        // bottom of screen as origin is centre
        let x_pos = 0.0;
        let y_pos = -screen_size.1 * 0.9;

        println!("ground position: {:?}", (x_pos, y_pos));

        let ground_width = screen_size.0;
        let ground_height = 64.;

        GroundBundle {
            ground: Ground {},
            transform: Transform::from_xyz(x_pos, y_pos, 0.0),
            sprite: Sprite::from_image(asset_server.load("sprites/ground_wide.png")),
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(ground_width, ground_height),
            gravity_scale: GravityScale(0.0),
        }
    }
}
