use crate::game_manager::{GameState, components::GameManager, systems::check_timer};
use bevy::prelude::*;

pub(crate) struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, check_timer);
        app.init_state::<GameState>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(GameManager::new());
}
