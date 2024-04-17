use bevy::prelude::*;

mod r#macro;

mod eye;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(eye::EyePlugin);
    }
}
