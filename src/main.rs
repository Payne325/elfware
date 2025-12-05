mod counting;

use bevy::prelude::*;

use crate::counting::CountingPlugin;

fn windows_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Elfware".into(),
            name: Some("elfware.app".into()),
            resolution: (800, 600).into(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
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
    app.add_plugins(CountingPlugin);

    app.add_systems(Startup, setup);
    app.insert_resource(ClearColor(MAGENTA));
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
