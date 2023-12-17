// #![allow(unused)]

mod assets;
mod camera;
mod map;
mod player;
mod state;

use assets::AssetsPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy 2d game".to_string(),
                resolution: WindowResolution::new(1080., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::hex("#6d707b").unwrap()))
        .add_plugins((
            WorldInspectorPlugin::new().run_if(input_toggle_active(true, KeyCode::Escape)),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            CameraPlugin,
            MapPlugin,
            StatePlugin,
            AssetsPlugin,
            PlayerPlugin,
        ))
        .run();
}
