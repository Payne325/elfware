use bevy::prelude::*;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// All new games need an entry here.
#[derive(Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub(crate) enum MiniGame {
    Chase,
    Santa,
}

#[derive(Component)]
pub(crate) struct GameManager {
    timer: Timer,
    waiting_to_start: bool,
    current_game: MiniGame,
    game_iter: MiniGameIter,
}

impl GameManager {
    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            waiting_to_start: true,
            current_game: MiniGame::Chase,
            game_iter: MiniGame::iter(),
        }
    }

    pub(crate) fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if self.timer.is_finished() {
            self.waiting_to_start = !self.waiting_to_start;

            if self.waiting_to_start {
                match self.game_iter.next() {
                    Some(game) => self.current_game = game,
                    None => {
                        self.current_game = MiniGame::Chase;
                        self.game_iter = MiniGame::iter();
                    }
                }
            }
        }
    }

    pub(crate) fn should_start(&self) -> bool {
        self.timer.is_finished() && self.waiting_to_start
    }

    pub(crate) fn should_end(&self) -> bool {
        self.timer.is_finished() && !self.waiting_to_start
    }

    pub(crate) fn current_game(&self) -> MiniGame {
        self.current_game.clone()
    }
}

#[derive(Event)]
pub struct StartGame(pub(crate) MiniGame);

#[derive(Event)]
pub struct EndGame(pub(crate) MiniGame);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    On(MiniGame),
    #[default]
    Off,
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
