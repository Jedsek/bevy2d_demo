use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands) {
    let ground_size = 840.0;
    let ground_height = 10.0;

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(ground_size, ground_height),
        SpriteBundle {
            sprite: Sprite {
                color: Color::hex("#292a33").unwrap(),
                custom_size: Some(Vec2::new(2. * ground_size, 2. * ground_height)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(10., 10.),
        SpriteBundle {
            sprite: Sprite {
                color: Color::hex("#3c434d").unwrap(),
                custom_size: Some(Vec2::new(20., 20.)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 300., 0.0),
            // .with_rotation(Quat::from_rotation_z(PI / 10.0)),
            ..default()
        },
        GravityScale(7.),
        Velocity {
            linvel: Vec2::new(4.0, -1.0),
            angvel: -3.0,
        },
    ));
}
