use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{player::PlayerState, state::AppState};

#[rustfmt::skip]
#[derive(Resource, AssetCollection)]
pub struct MyAssets {
    #[asset(path = "sprite_sheet/Samurai/Idle.png")]
    #[asset(texture_atlas(
        tile_size_x = 48., tile_size_y = 84., columns = 6, rows = 1, padding_x = 80., padding_y = 44., offset_x = 42., offset_y = 44.,
    ))]
    pub player_idle: Handle<TextureAtlas>,

    #[asset(path = "sprite_sheet/Samurai/Run.png")]
    #[asset(texture_atlas(
        tile_size_x = 51., tile_size_y = 80., columns = 8, rows = 1, padding_x = 77., padding_y = 48., offset_x = 16., offset_y = 48.,
    ))]
    pub player_run: Handle<TextureAtlas>,

    #[asset(path = "sprite_sheet/Samurai/Walk.png")]
    #[asset(texture_atlas(
        tile_size_x = 51., tile_size_y = 80., columns = 8, rows = 1, padding_x = 77., padding_y = 48., offset_x = 16., offset_y = 48.,
    ))]
    pub player_walk: Handle<TextureAtlas>,


    #[asset(path = "sprite_sheet/Samurai/Jump.png")]
    #[asset(texture_atlas(
        tile_size_x = 64., tile_size_y = 78., columns = 12, rows = 1, padding_x = 65., padding_y = 50., offset_x = 28., offset_y = 50.,
    ))]
    pub player_jump: Handle<TextureAtlas>,

    #[asset(path = "sprite_sheet/Samurai/Attack_11.png")]
    #[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 6, rows = 1))]
    pub player_attack_1: Handle<TextureAtlas>,

    #[asset(path = "sprite_sheet/Samurai/Attack_2.png")]
    #[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 6, rows = 1))]
    pub player_attack_2: Handle<TextureAtlas>,

    #[asset(path = "sprite_sheet/Samurai/Attack_3.png")]
    #[asset(texture_atlas(tile_size_x = 128., tile_size_y = 128., columns = 6, rows = 1))]
    pub player_attack_3: Handle<TextureAtlas>,

    #[asset(path = "audio/game-loop-music.wav")]
    pub background_music: Handle<AudioSource>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading);
    }
}

impl MyAssets {
    pub fn get_atlas(&self, state: PlayerState) -> Handle<TextureAtlas> {
        match state {
            PlayerState::Idle => self.player_idle.clone(),
            PlayerState::Run => self.player_run.clone(),
            PlayerState::Walk => self.player_walk.clone(),
            PlayerState::Jump => self.player_jump.clone(),
            PlayerState::Attack => self.player_attack_3.clone(),
            // _ => todo!(),
        }
    }
}
