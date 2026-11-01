use bevy::prelude::*;
use rand::RngExt;

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 8;

const GRID: Vec2 = Vec2::new(GRID_WIDTH as f32, GRID_HEIGHT as f32);
const CELL_SIZE: f32 = 40.0;

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
        .init_state::<Phase>()
        .add_systems(Startup, setup.spawn())
        // Choosing
        .add_systems(OnEnter(Phase::Choosing), spawn_hand.spawn())
        // .add_systems(
        //     Update,
        //     (move_hand, rotate_hand, drop_hand)
        //         .chain()
        //         .run_if(in_state(Phase::Choosing)),
        // )
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum Phase {
    #[default]
    Choosing,
    Falling,
    Merging,
}

type ItemKind = u8;

#[derive(Component, Clone, Copy, PartialEq, Default)]
struct Item(ItemKind);

#[derive(Component, Clone, Copy, PartialEq, Default)]
struct Position(i32, i32);

impl Position {
    fn to_world(&self) -> Vec3 {
        let Position(x, y) = *self;

        let top_left = Vec3::from((GRID, 0.0)) * -0.5;
        let pos = Vec3::new(x as f32, y as f32, 0.0);
        return (top_left + pos) * CELL_SIZE + CELL_SIZE * 0.5;
    }
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
    let background_size = GRID * CELL_SIZE;
    bsn! {
        #Cauldron
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.15),
            custom_size: background_size,
        }
    }
}

fn spawn_hand() -> impl SceneList {
    let mut rng = rand::rng();
    let item_a = Item(rng.random_range(0..3));
    let item_b = Item(rng.random_range(0..3));

    let pos_a = Position(2, GRID_HEIGHT as i32 + 1);
    let pos_b = Position(2, GRID_HEIGHT as i32 + 2);
    bsn_list! {
        item_a
        Text2d::new(item_a.0.to_string())
        pos_a
        Transform::from_translation(pos_a.to_world())
        --
        item_b
        Text2d::new(item_b.0.to_string())
        pos_b
        Transform::from_translation(pos_b.to_world())
    }
}

// Falling
// - on start
//   - add Moving { from , to } // Tween
// - update
//   - move to `to` position with tweening - all have the same speed / velocity
//   - remove Moving once finished
//   - if no Moving - go to merging phase

// Merging
// - on start
//   - find 3+ mergable items => add BeingMerged { with_elements: Vec[Entity], end_pos: Pos }
//   - if none => go to phase PlayerChoosing
// - update:
//   - play animation for merging
//     - 1. elements disappear
//       - elements go to white then transparent
//       - bubble particules spawns and speard out
//     - 2. bubble moving
//       - bubbles moves toward destination and bloupbloup
//     - 3. new element appearing
//       - new element spawn white then colored
//       - white circle shockwave + slight bubble particules explosing out

//   - drop 3 entity elements and spawn a new one at the right position
//   - go to falling phase
