//use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum ChaseGameState {
    On,
    #[default]
    Off,
}

pub(crate) struct ChasePlugin;

impl Plugin for ChasePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observe_game_start);
        app.add_observer(observe_game_end);
        app.add_systems(Update, run_game.run_if(in_state(ChaseGameState::On)));
        app.init_state::<ChaseGameState>();
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
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.spawn((
        Elf {},
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(asset_server.load("sprites/elf.png")),
        //RigidBody::Dynamic,
        //add collider here
    ));
    println!("BADABING!");
    toggle_game_state(state, next_state);
}

fn observe_game_end(
    _event: On<EndChaseGame>,
    mut commands: Commands,
    elf: Single<Entity, With<Elf>>,
    state: Res<State<ChaseGameState>>,
    next_state: ResMut<NextState<ChaseGameState>>,
) {
    commands.entity(elf.entity()).despawn();
    println!("BADABONG!");
    toggle_game_state(state, next_state);
}

fn run_game(
    mut elf: Single<&mut Transform, With<Elf>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 200.0;

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
        elf.translation += direction.normalize() * speed * time.delta_secs();
    }
}

fn toggle_game_state(
    state: Res<State<ChaseGameState>>,
    mut next_state: ResMut<NextState<ChaseGameState>>,
) {
    match state.get() {
        ChaseGameState::On => next_state.set(ChaseGameState::Off),
        ChaseGameState::Off => next_state.set(ChaseGameState::On),
    }
}
