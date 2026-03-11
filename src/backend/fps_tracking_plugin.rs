use bevy::prelude::*;

use crate::backend::*;

pub struct FpsTrackingPlugin;

impl Plugin for FpsTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FPS(0.0));
        app.add_systems(Update, sync_with_ui);
    }
}

fn sync_with_ui(mut fps: ResMut<FPS>, time: Res<Time>) {
    let new_fps = 1000.0 / time.delta().as_millis() as f32;
    *fps = FPS(new_fps);
}
