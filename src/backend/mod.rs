use std::fmt::Display;

use bevy::ecs::prelude::*;
use bevy::prelude::{Deref, DerefMut};

pub mod base_plugin;
pub mod fps_tracking_plugin;
pub mod idle_time_plugin;
pub mod player_plugin;
pub mod sphere;

#[derive(Resource, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct CurrentIdleTimeSeconds(pub f64);

#[derive(Resource, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct LongestIdleTimeSeconds(pub f64);

#[derive(Resource, Default, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct AutomationSpeed {
    #[deref]
    pub speed: f64,
    pub raw_speed: f64,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FPS(pub f32);

impl Display for FPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for CurrentIdleTimeSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} sec", self.0)
    }
}

impl Display for LongestIdleTimeSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} sec", self.0)
    }
}

impl Display for AutomationSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} Hz", self.speed)
    }
}

impl AutomationSpeed {
    pub fn step_by(&mut self, value: f64) {
        if value > self.raw_speed {
            self.speed = value
        }

        self.raw_speed = value;
    }
}
