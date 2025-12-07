use crate::game_manager::{MiniGame, MiniGameIter};
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use std::time::Duration;
use strum::IntoEnumIterator;

#[derive(Component)]
pub(super) struct GameManager {
    timer: Timer,
    waiting_to_start: bool,
    current_game: MiniGame,
    game_iter: MiniGameIter,
}

impl GameManager {
    pub(super) fn new() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            waiting_to_start: true,
            current_game: MiniGame::Chase,
            game_iter: MiniGame::iter(),
        }
    }

    pub(super) fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if self.timer.is_finished() {
            self.waiting_to_start = !self.waiting_to_start;

            if self.waiting_to_start {
                match self.game_iter.next() {
                    Some(game) => {
                        println!("Next game is {game:?}");
                        self.current_game = game;
                    }
                    None => {
                        println!("Go round again");
                        self.game_iter = MiniGame::iter();
                        self.current_game = self.game_iter.next().unwrap_or_else(|| {
                            println!("DEBUG: GameManager Expected game iter to be some.");
                            MiniGame::Chase
                        });
                        println!("Next game is {:?}", self.current_game);
                    }
                }
            }
        }
    }

    pub(super) fn should_start(&self) -> bool {
        self.timer.is_finished() && self.waiting_to_start
    }

    pub(super) fn should_end(&self) -> bool {
        self.timer.is_finished() && !self.waiting_to_start
    }

    pub(super) fn current_game(&self) -> MiniGame {
        self.current_game.clone()
    }
}

#[derive(Component)]
pub(crate) struct MyMusic;

#[derive(Bundle)]
pub(crate) struct MusicBundle {
    player: AudioPlayer,
    playback_settings: PlaybackSettings,
    music_component: MyMusic,
}

impl MyMusic {
    pub(crate) fn new_bundle_once_and_cleanup(
        asset_server: &Res<AssetServer>,
        sound_path: impl Into<String>,
    ) -> MusicBundle {
        MusicBundle {
            player: AudioPlayer::new(asset_server.load(sound_path.into())),
            playback_settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::Linear(0.5),
                ..default()
            },
            music_component: MyMusic,
        }
    }
}
