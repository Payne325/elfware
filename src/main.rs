mod chase;
mod santa;

use crate::{chase::ChasePlugin, santa::SantaPlugin};
use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_input::common_conditions::input_just_pressed;
use std::time::Duration;

fn windows_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Elfware".into(),
            name: Some("elfware.app".into()),
            resolution: WindowResolution::new(1280, 720),
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}

const MAGENTA: Color = Color::linear_rgb(255., 0., 255.);

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(windows_settings())
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins(ChasePlugin);
    app.add_plugins(SantaPlugin);

    app.add_systems(Startup, setup);
    app.add_systems(Update, check_timer);
    app.add_systems(Update, esc.run_if(input_just_pressed(KeyCode::Escape)));
    app.insert_resource(ClearColor(MAGENTA));
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(MiniGameTimer::new());
}

fn esc(mut ev_exit: MessageWriter<AppExit>) {
    ev_exit.write(AppExit::Success);
}

#[derive(Component)]
struct MiniGameTimer {
    timer: Timer,
    waiting_to_start: bool,
}

impl MiniGameTimer {
    fn new() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
            waiting_to_start: true,
        }
    }

    fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);

        let finished = self.timer.is_finished();

        if finished {
            self.waiting_to_start = !self.waiting_to_start;
        }
    }

    fn should_start(&self) -> bool {
        self.timer.is_finished() && self.waiting_to_start
    }

    fn should_end(&self) -> bool {
        //debug
        false
        // self.timer.is_finished() && !self.waiting_to_start
    }
}

fn check_timer(
    mut commands: Commands,
    mut mini_game_timer: Single<&mut MiniGameTimer>,
    time: Res<Time>,
) {
    mini_game_timer.tick(time.delta());

    if mini_game_timer.should_start() {
        commands.trigger(santa::StartGame {});
    } else if mini_game_timer.should_end() {
        commands.trigger(santa::EndGame {});
    }
}
