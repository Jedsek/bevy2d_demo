use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Resource)]
pub struct Animation {
    pub timer: Timer,
    pub frame_count: usize,
}

impl Animation {
    pub fn new(duration_second: f32, frame_count: usize) -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(duration_second),
                TimerMode::Repeating,
            ),
            frame_count,
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, animate_player.run_if(in_state(AppState::Next)));
    }
}
