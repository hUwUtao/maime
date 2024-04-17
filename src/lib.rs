#![allow(clippy::type_complexity)]

use bevy::app::App;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;

use bevy::prelude::*;

mod controller;
mod view;

mod entity;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[allow(dead_code)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Transition,

    PadServer,

    SongSelect,

    #[default]
    Smash,

    Settings,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            controller::ControlPlugin,
            view::ViewPlugin,
            entity::ScenePlugin,
        ));

        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            // #[cfg(debug_assertions)]
            // {
            //     LogDiagnosticsPlugin::default()
            // },
        ));
    }
}

#[macro_export]
macro_rules! euler {
    ($x:expr, $y:expr, $z:expr) => {
        Quat::from_euler(EulerRot::XYZ, $x, $y, $z)
    };
}
