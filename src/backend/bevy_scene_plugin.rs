// use bevy::input::mouse::{MouseButton, MouseMotion};
use bevy::prelude::*;

use crate::backend::*;

#[derive(Component)]
pub struct OrbitCamera {
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            distance: 3.0,
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.01,
        }
    }
}
pub struct BevyScenePlugin;

impl Plugin for BevyScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FPS(0.0));
        // app.add_systems(Startup, setup);
        // app.insert_resource(CurrentIdleTimeSeconds(25.125));
        // app.insert_resource(LongestIdleTimeSeconds(30.0));
        app.add_systems(Update, sync_with_ui);
        // app.add_systems(Update, sync_with_ui_2);
    }
}

// fn setup(
//     mut commands: Commands,
//     //     mut meshes: ResMut<Assets<Mesh>>,
//     //     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // commands.spawn((IdleTimeSeconds(25.125), CurrentIdleTime));
//     // commands.spawn((IdleTimeSeconds(30.0), BestIdleTime));
//     commands.spawn(TestComponent(0));
//     commands.spawn(TestComponent(1));
//     commands.spawn(TestComponent(2));
//     commands.spawn(TestComponent(3));
//     commands.spawn(TestComponent(4));
//     commands.spawn(TestComponent(5));
//     commands.spawn(TestComponent(6));
//     commands.spawn(TestComponent(7));
// }

fn sync_with_ui(mut fps: ResMut<FPS>, time: Res<Time>) {
    let new_fps = 1000.0 / time.delta().as_millis() as f32;
    *fps = FPS(new_fps);
}

// fn sync_with_ui_2(
//     mut cur_time: ResMut<CurrentIdleTimeSeconds>,
//     mut longest_time: ResMut<LongestIdleTimeSeconds>,
//     time: Res<Time>,
// ) {
//     **cur_time += time.delta().as_secs_f64();
//
//     if **cur_time > **longest_time {
//         **longest_time = **cur_time;
//     }
// }
