use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct GroundTexture {
    pub(super) handle: Handle<Image>,
    pub(super) applied: bool, // track if sampler has already been applied
}
