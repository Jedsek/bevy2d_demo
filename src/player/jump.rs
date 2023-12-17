use super::{animation::Animation, PlayerInfo, PlayerState};
use crate::player::Player;
use crate::state::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
struct JumpAnimation(Animation);

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

pub struct JumpPlugin;

impl Plugin for JumpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(JumpAnimation(Animation::new(0.1, 12)))
            .add_systems(
                Update,
                (player_jump, jump_reset, animate_jump).run_if(in_state(AppState::Next)),
            );
    }
}

fn player_jump(
    mut query: Query<
        (
            &mut Jumper,
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut PlayerInfo,
        ),
        With<Player>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut jumper, mut controller, output, mut player_info) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::W) && !jumper.is_jumping {
        jumper.is_jumping = true;
    }
    if jumper.is_jumping {
        player_info.update_state(PlayerState::Jump);
    }
    if let Some(Vec2 { x, y }) = controller.translation {
        if output.grounded && y <= jumper.jump_impulse {
            controller.translation = Some(Vec2::new(x, y + 2.0));
        } else {
            jumper.is_jumping = false;
        }
    }
}

fn jump_reset(mut query: Query<(&mut Jumper, &KinematicCharacterControllerOutput), With<Player>>) {
    for (mut jumper, output) in &mut query {
        jumper.is_jumping = !output.grounded;
    }
}

fn animate_jump(
    mut query: Query<(&mut TextureAtlasSprite, &PlayerInfo, &Jumper), With<Player>>,
    mut animation: ResMut<JumpAnimation>,
    time: Res<Time>,
) {
    let (mut sprite, player_info, jumper) = query.single_mut();

    if player_info.state == PlayerState::Jump {
        animation.0.timer.tick(time.delta());
        if animation.0.timer.finished()
            && (sprite.index + 1) < animation.0.frame_count - 1
            && jumper.is_jumping
        {
            sprite.index = (sprite.index + 1) % animation.0.frame_count;
        }
    }
}
