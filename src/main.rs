use bevy::prelude::*;

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 8;
const CELL_SIZE: usize = 50;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(AssetPlugin {
        //     // Wasm builds will check for meta files (that don't exist) if this isn't set.
        //     // This causes errors and even panics in web builds on itch.
        //     // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        //     meta_check: AssetMetaCheck::Never,
        //     ..default()
        // }))
        .add_systems(Startup, setup.spawn())
        .run();
}

fn setup() -> impl SceneList {
    bsn_list! {
        #Camera
        Camera2d
        --
        @cauldron()
    }
}

fn cauldron() -> impl Scene {
    let background_size = Vec2::new(GRID_WIDTH as f32, GRID_HEIGHT as f32) * CELL_SIZE as f32;
    bsn! {
        #Cauldron
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.15),
            custom_size: background_size,
        }
    }
}
