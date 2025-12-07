use crate::game_manager::{EndGame, MyMusic};
use bevy::prelude::*;

pub(super) fn despawn_sound(
    _event: On<EndGame>,
    mut commands: Commands,
    query_music: Query<Entity, With<MyMusic>>,
) {
    for entity in query_music.iter() {
        commands.entity(entity).despawn();
    }
}
