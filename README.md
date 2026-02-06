# My Personal Template for Game Jams

This is forked from the [official template](https://github.com/bevyengine/bevy_github_ci_template), and I'm adding the things I usually use and are a pain to set up.

## Plugins used in this template

Bevy plugins by yours truly:

* [bevy-yoleck](https://github.com/idanarye/bevy-yoleck) - level editing
* [bevy-egui-kbgp](https://github.com/idanarye/bevy-egui-kbgp) - menu navigation

Bevy plugins by other people:

* [bevy_egui](https://github.com/vladbat00/bevy_egui) - UI for the menu (also used by Yoleck for the level editing UI)
* [bevy_pkv](https://github.com/johanhelsing/bevy_pkv) - save/load level progress

## Stuff you need to edit

* `package.name` in `Cargo.toml` - goes without saying.
* `env` variables in `.github/workflows/release.yaml` - see their comments or refer to the README in [original template](https://github.com/bevyengine/bevy_github_ci_template).
* Global consts at `src/lib.rs`:
  * `GAME_TITLE_FOR_MENU` - exactly what it says on the tin
  * `ORG_AND_APP_NAMES` - sets the namespace under which bevy_pkv saves the level progress data
* Probably want to delete the `KeyCode::End` binding in `src/main.rs` since it allows skipping a level.

## Yoleck Level Editing

To launch in level editor mode:

```bash
cargo run -- --editor
```

To start the game at a specific level, skipping both the main menu and the level progress (meaning you can play levels that weren't reached through gameplay):
```bash
cargo run -- --level <Level-Name-Without-.yol-Suffix>
```

### Arena (platforms)

`src/arena.rs` defines two types of platforms you can add in the level editor:

* `BoxPlatform` - which you can freely resize. It scales the transform, so if you use a texture it'd get messed up.
* `BrickPlugin` - both its location and size are snapped to a grid, using a mechanism implemented in `src/yoleck_ext/align_to_grid.rs`. Instead of a single PBR, its body is combosed of many identical objects (bricks) so it can properly deal with textured models.

Both can be resised in the level editor by dragging the green borders that appear when they are selected. This mechanism is implemented in `src/yoleck_ext/resize_knobs.rs`.

Actual games probably only need one of them, so you can delete the other.

The `populate_...` systems in the `Arena` create their PBR - edit these to create your own graphics.

## Menu

The menu is defined in `src/menu.rs`. The main part is s `chain()` of systems running in the `EguiPrimaryContextPass` - these systems draw the menu. To add a new section to the menu just add a system to that chain and have it use `ResMut<FrameUi>` to get access to egui.

## Menu Navigation

Look for `KbgpNavBindings` in `src/main.rs`.

## Level Progress

Implemented in `src/level_handling.rs`.

When the player completed the level, set `ResMut<NextState<AppState>>` to `AppState::LevelCompleted` - this will open the level selection menu with the next level unlocked. Since this template does not have any gameplay condition for finishing the levels, it adds a binding to the `End` key for testing level completion.

If the player dies or otherwise fails a level:

* Set `ResMut<NextState<AppState>>` to `AppState::GameOver`.
* Set `ResMut<GameOverReason>` to the reason why the player lost (edit `GameOverReason` to have the reasons relevant to your game)
