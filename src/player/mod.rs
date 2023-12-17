mod animation;
mod jump;
mod movement;

use crate::{assets::MyAssets, state::AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(dead_code)]
#[derive(Default, Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
    Walk,
    Jump,
    Attack,
}

#[derive(Component, Debug)]
struct PlayerInfo {
    state: PlayerState,
    prev_state: PlayerState,
}

impl PlayerInfo {
    fn new(state: PlayerState) -> Self {
        Self {
            state,
            prev_state: state,
        }
    }

    fn is_state_changed(&self) -> bool {
        self.state != self.prev_state
    }

    fn update_state(&mut self, new_state: PlayerState) {
        self.prev_state = self.state;
        self.state = new_state;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // jump::JumpPlugin,
            movement::MovementPlugin,
            // animation::AnimationPlugin,
            // state::PlayerStatePlugin,
            // attack::AttackPlugin,
        ))
        .add_systems(OnEnter(AppState::Next), spawn_player)
        .add_systems(
            Update,
            (toggle_player_state).run_if(in_state(AppState::Next)),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    texture_atlas: Res<Assets<TextureAtlas>>,
) {
    let startup_state = PlayerState::Run;
    let atlas_handle = my_assets.get_atlas(startup_state);

    let player_size = texture_atlas.get(&atlas_handle).unwrap().size;
    info!("{:?}", player_size);
    let player_x = 48.;
    let player_y = 84.;

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: atlas_handle,
            transform: Transform::from_xyz(-100., 100., 0.),
            ..default()
        },
        (PlayerInfo::new(startup_state),),
        (
            RigidBody::KinematicVelocityBased,
            KinematicCharacterController {
                translation: Some(Vec2::new(0., 0.)),
                apply_impulse_to_dynamic_bodies: false,
                offset: CharacterLength::Absolute(0.01),
                snap_to_ground: Some(CharacterLength::Relative(0.2)),
                ..default()
            },
            // Collider::cuboid(player_x / 2., player_y / 2.),
            Collider::round_cuboid(player_x / 24., player_y / 4., 0.22),
            // Collider::ball(40.),
            Ccd::enabled(),
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
        ),
        Player,
    ));
}

fn toggle_player_state(
    mut query: Query<
        (
            &PlayerInfo,
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    my_assets: Res<MyAssets>,
) {
    let (player_info, mut atlas_handle, mut sprite) = query.single_mut();

    if !player_info.is_state_changed() {
        return;
    }

    sprite.index = 0;
    *atlas_handle = my_assets.get_atlas(player_info.state);

    // todo!()
}
