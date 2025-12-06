mod chase;
mod game_manager;
mod santa;

use crate::{chase::ChasePlugin, santa::SantaPlugin};
use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_input::common_conditions::input_just_pressed;

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
    app.insert_resource(Gravity(Vec2::NEG_Y * 500.));
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(game_manager::GameManager::new());
}

fn esc(mut ev_exit: MessageWriter<AppExit>) {
    ev_exit.write(AppExit::Success);
}

fn check_timer(
    mut commands: Commands,
    mut game_manager: Single<&mut game_manager::GameManager>,
    time: Res<Time>,
) {
    game_manager.tick(time.delta());

    if game_manager.should_start() {
        match game_manager.current_game() {
            game_manager::MiniGame::Chase => commands.trigger(chase::StartGame {}),
            game_manager::MiniGame::Santa => commands.trigger(santa::StartGame {}),
        }
    } else if game_manager.should_end() {
        match game_manager.current_game() {
            game_manager::MiniGame::Chase => commands.trigger(chase::EndGame {}),
            game_manager::MiniGame::Santa => commands.trigger(santa::EndGame {}),
        };
    }
}
