use std::fmt::Display;

use bevy::ecs::prelude::*;
use bevy::prelude::{Deref, DerefMut};

pub mod base_plugin;
pub mod bevy_scene_plugin;
pub mod idle_time_plugin;
pub mod sphere;

#[derive(Resource, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct CurrentIdleTimeSeconds(pub f64);

#[derive(Resource, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct LongestIdleTimeSeconds(pub f64);

#[derive(Resource, Debug, Clone)]
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

#[derive(Component, Debug, Clone, Deref, DerefMut)]
pub struct TestComponent(pub usize);
