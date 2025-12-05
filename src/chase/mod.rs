//use avian2d::prelude::*;
use bevy::prelude::*;

pub(crate) struct ChasePlugin;

impl Plugin for ChasePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observe_game_start);
        app.add_observer(observe_game_end);
    }
}

#[derive(Event)]
pub struct StartChaseGame {}

#[derive(Event)]
pub struct EndChaseGame {}

#[derive(Component)]
pub struct Elf {}

fn observe_game_start(
    _event: On<StartChaseGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Elf {},
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(asset_server.load("sprites/elf.png")),
        //RigidBody::Dynamic,
        //add collider here
    ));
    println!("BADABING!")
}

fn observe_game_end(
    _event: On<EndChaseGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
) {
    commands.entity(elf.entity()).despawn();
    println!("BADABONG!")
}
