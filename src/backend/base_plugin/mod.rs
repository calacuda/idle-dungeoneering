use std::time::Instant;

use bevy::prelude::*;

use crate::backend::idle_time_plugin::IdleTimePlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, States)]
pub enum MainGameStates {
    /// start screen rendered in Dioxus, which will communicate with the Bevy Backend telling it to
    /// change states.
    #[default]
    StartScreen,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, States)]
pub enum AutomationStates {
    #[default]
    Manual,
    Automation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Resource)]
pub struct StartTime(pub Instant);

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainGameStates>();
        app.init_state::<AutomationStates>();
        app.add_plugins(IdleTimePlugin);
        app.add_systems(Startup, |mut cmds: Commands| {
            cmds.insert_resource(StartTime(Instant::now()))
        });
    }
}
