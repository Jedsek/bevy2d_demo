use bevy::prelude::*;

use crate::{player::Player, state::AppState};

#[derive(Component)]
struct Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow.run_if(in_state(AppState::Next)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Camera,
    ));
}

fn camera_follow(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<(&Transform, &TextureAtlasSprite), With<Player>>,
) {
    let mut camera_pos = camera.single_mut();
    let (player_pos, sprite) = player.single();

    let mut delta_x = 80.;
    if sprite.flip_x {
        delta_x *= -1.
    };

    let mut target_pos = player_pos.translation;
    target_pos.x += delta_x;
    target_pos.y += 100.;

    let new_pos = camera_pos.translation.lerp(target_pos, 0.08);

    camera_pos.translation = new_pos;
}
