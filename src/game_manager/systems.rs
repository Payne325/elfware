use crate::{
    background::ChangeBackground,
    game_manager::{EndGame, GameState, MiniGame, StartGame, components::GameManager},
};
use bevy::prelude::*;

pub(super) fn check_timer(
    mut commands: Commands,
    mut game_manager: Single<&mut GameManager>,
    time: Res<Time>,
) {
    game_manager.tick(time.delta());

    if game_manager.should_start() {
        match game_manager.current_game() {
            MiniGame::Chase => {
                commands.trigger(StartGame(MiniGame::Chase));
                commands.trigger(ChangeBackground::custom("sprites/chase_background.png"));
            }
            MiniGame::Santa => {
                commands.trigger(StartGame(MiniGame::Santa));
                commands.trigger(ChangeBackground::custom("sprites/santa_background.png"));
            }
        }
    } else if game_manager.should_end() {
        match game_manager.current_game() {
            MiniGame::Chase => commands.trigger(EndGame(MiniGame::Chase)),
            MiniGame::Santa => commands.trigger(EndGame(MiniGame::Santa)),
        };

        commands.trigger(ChangeBackground::title());
    }
}

pub(crate) fn toggle_game_state(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    game: MiniGame,
) {
    match state.get() {
        GameState::On(_) => next_state.set(GameState::Off),
        GameState::Off => next_state.set(GameState::On(game)),
    }
}
