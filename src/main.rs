use bevy::prelude::*;
use rand::RngExt;

mod hand;

const GRID_WIDTH: i32 = 5;
const GRID_HEIGHT: i32 = 8;

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
        .add_systems(OnEnter(Phase::Choosing), hand::spawn_hand.spawn())
        .add_systems(
            Update,
            (hand::move_hand, hand::rotate_hand, hand::drop_hand)
                .chain()
                .run_if(in_state(Phase::Choosing)),
        )
        .add_systems(Update, (update_world_pos, hand::update_hand_world_pos))
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

pub const MOVEMENT_SPEED: f32 = 300.0;
pub const CLOSE_ENOUGH_DISTANCE: f32 = 10.0;

fn update_world_pos(positions: Query<(&Position, &mut Transform)>, time: Res<Time>) {
    for (pos, mut transform) in positions {
        let movement = pos.to_world() - transform.translation;
        if movement.length_squared() <= CLOSE_ENOUGH_DISTANCE {
            continue;
        }
        let direction = movement.normalize() * time.delta_secs() * MOVEMENT_SPEED;
        transform.translation += direction;
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
