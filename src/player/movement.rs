use std::time::Duration;

use super::animation::Animation;
use super::PlayerInfo;
use crate::player::Player;
use crate::{player::PlayerState, state::AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const RUN_SPEED: f32 = 3.0;
const GRAVITY_VELOCITY: f32 = 0.3;
const FRICTION_VELOCITY: f32 = 0.1;

#[derive(Resource)]
struct RunAnimation(Animation);

#[derive(Resource)]
struct WalkAnimation(Animation);

#[derive(Resource)]
struct IdleAnimation(Animation);

#[derive(Resource)]
struct RunTimer(Timer);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RunAnimation(Animation::new(0.09, 8)))
            .insert_resource(WalkAnimation(Animation::new(0.1, 8)))
            .insert_resource(IdleAnimation(Animation::new(0.1, 6)))
            .insert_resource(RunTimer(Timer::new(
                Duration::from_secs_f32(0.5),
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (movement_player, animate_run, animate_walk, animate_idle)
                    .run_if(in_state(AppState::Next)),
            )
            .add_systems(Update, emulate_gravity_and_friction.before(movement_player));
    }
}

// #[rustfmt::skip]
fn movement_player(
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &mut PlayerInfo,
            &mut TextureAtlasSprite,
            &KinematicCharacterControllerOutput,
        ),
        With<Player>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut controller, mut player_info, mut sprite, output) in &mut query {
        let Vec2 { x, y } = output.effective_translation;

        let jumper_factor = if y.abs() <= 0.1 { 1.0 } else { 0.75 };
        let toward_factor = if sprite.flip_x { -1.0 } else { 1.0 };

        if output.grounded {
            if keyboard_input.any_pressed([KeyCode::D, KeyCode::A]) {
                let mut speed = RUN_SPEED * jumper_factor * toward_factor;
                if keyboard_input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
                    speed *= 2.;
                }
                controller.translation = Some(Vec2::new(speed, y));
                player_info.update_state(PlayerState::Run);
            } else if x.abs() == 0. {
                player_info.update_state(PlayerState::Idle);
            } else if !keyboard_input.any_pressed([KeyCode::D, KeyCode::A]) {
                player_info.update_state(PlayerState::Walk);
            }
        }

        if keyboard_input.pressed(KeyCode::D) {
            sprite.flip_x = false;
        } else if keyboard_input.pressed(KeyCode::A) {
            sprite.flip_x = true;
        }
        info!("{}, {}", x, y)
    }
}

fn emulate_gravity_and_friction(
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    for (mut controller, output, _sprite) in &mut query {
        let Vec2 { mut x, mut y } = output.effective_translation;

        if !output.grounded {
            y -= GRAVITY_VELOCITY;
        };

        // let toward_right = !sprite.flip_x;
        // let toward_left = !toward_right;
        // let velocity_right = x > 0.;
        // let velocity_left = !velocity_right;

        // let f = if toward_right && velocity_right {
        //     -1.
        // } else if toward_right && velocity_left {
        //     1.
        // };
        if x.abs() <= FRICTION_VELOCITY {
            if output.grounded {
                x = 0.
            }
        } else {
            let f = if x > 0. { -1. } else { 1. };
            x += FRICTION_VELOCITY * f;
        }
        controller.translation = Some(Vec2::new(x, y));
    }
}

fn animate_run(
    mut query: Query<(&mut TextureAtlasSprite, &PlayerInfo), With<Player>>,
    mut animation: ResMut<RunAnimation>,
    time: Res<Time>,
) {
    let (mut sprite, player_info) = query.single_mut();

    let state = player_info.state;
    if state == PlayerState::Run {
        animation.0.timer.tick(time.delta());
        if animation.0.timer.finished() {
            sprite.index = (sprite.index + 1) % animation.0.frame_count;
        }
    }
}

fn animate_walk(
    mut query: Query<(&mut TextureAtlasSprite, &PlayerInfo), With<Player>>,
    mut animation: ResMut<WalkAnimation>,
    time: Res<Time>,
) {
    let (mut sprite, player_info) = query.single_mut();

    let state = player_info.state;
    if state == PlayerState::Walk {
        animation.0.timer.tick(time.delta());
        if animation.0.timer.finished() {
            sprite.index = (sprite.index + 1) % animation.0.frame_count;
        }
    }
}

fn animate_idle(
    mut query: Query<(&mut TextureAtlasSprite, &PlayerInfo), With<Player>>,
    mut animation: ResMut<IdleAnimation>,
    time: Res<Time>,
) {
    let (mut sprite, player_info) = query.single_mut();

    if player_info.state == PlayerState::Idle {
        animation.0.timer.tick(time.delta());
        if animation.0.timer.finished() {
            sprite.index = (sprite.index + 1) % animation.0.frame_count;
        }
    }
}
