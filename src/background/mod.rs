use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct ChangeBackground {
    path: String,
}

impl ChangeBackground {
    pub(crate) fn title() -> Self {
        Self {
            path: "sprites/title_screen.png".to_string(),
        }
    }

    pub(crate) fn custom(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

#[derive(Component)]
struct Background {}

fn observe_background_change(
    event: On<ChangeBackground>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_background: Query<Entity, With<Background>>,
) {
    if let Ok(background) = query_background.single() {
        commands.entity(background).despawn();
    }

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -100.0),
        Sprite::from_image(asset_server.load(&event.path)),
    ));
}

pub(crate) struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observe_background_change);
    }
}
