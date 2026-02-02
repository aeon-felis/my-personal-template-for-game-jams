use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_github_ci_template::GameMainPlugin;
use bevy_yoleck::vpeol_3d::{Vpeol3dPluginForEditor, Vpeol3dPluginForGame};
use bevy_yoleck::{YoleckPluginForEditor, YoleckPluginForGame};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

fn main() -> AppExit {
    let args = Args::parse();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    // Egui is required both for Yoleck and for the menu system
    app.add_plugins(EguiPlugin::default());

    if args.editor {
        app.add_plugins((YoleckPluginForEditor, Vpeol3dPluginForEditor::topdown()));
    } else {
        app.add_plugins((YoleckPluginForGame, Vpeol3dPluginForGame));
    }

    app.add_plugins(GameMainPlugin {
        is_editor: args.editor,
        start_at_level: args.level,
    });

    app.run()
}
