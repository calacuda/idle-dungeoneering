//! minimal example showing each of the hooks

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_dioxus_sync::{panels::DioxusPanel, plugins::DioxusPlugin};

use crate::{
    backend::{base_plugin::BasePlugin, bevy_scene_plugin::BevyScenePlugin, sphere::SpherePlugin},
    frontend::AppUi,
};

pub mod backend;
pub mod frontend;

pub fn main() {
    let filter = format!("info,{}=trace", env!("CARGO_PKG_NAME").replace("-", "_"));
    let level = Level::INFO;

    let default_plugins = DefaultPlugins.set(LogPlugin {
        // Set the default log level for everything
        level,
        // Or use a filter string for fine-grained control
        filter: filter.clone(),
        ..default()
    });

    #[cfg(feature = "headless_ci")]
    let default_plugins = default_plugins
        .disable::<bevy::window::WindowPlugin>()
        .disable::<bevy::render::RenderPlugin>();

    App::new()
        .add_plugins((default_plugins, FrameTimeDiagnosticsPlugin::default()))
        .add_plugins(BevyScenePlugin)
        .add_plugins(SpherePlugin)
        .add_plugins(DioxusPlugin {
            bevy_info_refresh_fps: 30,
            main_window_ui: Some(DioxusPanel::new(AppUi {})),
        })
        .add_plugins(BasePlugin)
        .add_systems(Startup, move || {
            warn!("default log level is: {level}");
            warn!("default log filter: \"{filter}\"");
        })
        .run();
}

// fn show_log_filter(log_state: Res<LogDiagnosticsState>) {}
