mod arena;
mod utils;

use bevy::prelude::*;
use bevy_yoleck::prelude::{YoleckLoadLevel, YoleckSyncWithEditorState};
use bevy_yoleck::vpeol::prelude::*;

use self::arena::ArenaPlugin;

pub struct GameMainPlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for GameMainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_plugins(ArenaPlugin);

        if self.is_editor {
            app.add_plugins(YoleckSyncWithEditorState {
                when_editor: AppState::Editor,
                when_game: AppState::Game,
            });
        } else {
            // Menu and level loading plugins go here
            if let Some(start_at_level) = &self.start_at_level {
                let start_at_level = if start_at_level.ends_with(".yol") {
                    start_at_level.clone()
                } else {
                    format!("{}.yol", start_at_level)
                };
                app.add_systems(
                    Startup,
                    move |mut commands: Commands, asset_server: Res<AssetServer>| {
                        // TODO: use the level progress mechanism once I add it
                        commands.spawn(YoleckLoadLevel(
                            asset_server.load(format!("levels/{start_at_level}")),
                        ));
                    },
                );
            }
        }
    }
}

// TOOD: move this somewhere more fitting
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 16.0, 40.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        VpeolCameraState::default(),
        Vpeol3dCameraControl::topdown(),
    ));
}

#[derive(States, Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    // MainMenu,
    // PauseMenu,
    // LevelSelectMenu,
    // LoadLevel,
    Editor,
    Game,
    // LevelCompleted,
    // GameOver,
}
