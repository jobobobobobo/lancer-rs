use avian3d::prelude::PhysicsDebugPlugin;
use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct RenderPlugin;

mod camera;

// If the headless server can't run it or doesn't need it
// It goes in this plugin
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            //PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::default(),
        ));
    }
}
