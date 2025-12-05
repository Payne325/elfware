use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum ChaseGameState {
    On,
    #[default]
    Off,
}
