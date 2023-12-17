use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,
    Next,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>().add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::Next),
        );
    }
}
