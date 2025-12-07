use crate::game_manager::{
    GameState, MyMusic, components::GameManager, observers::despawn_sound, systems::check_timer,
};
use bevy::prelude::*;

pub(crate) struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, check_timer);
        app.add_observer(despawn_sound);
        app.init_state::<GameState>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(GameManager::new());
    commands.spawn(MyMusic::new_bundle_once_and_cleanup(
        &asset_server,
        "audio/super_elfware.mp3",
    ));
}
