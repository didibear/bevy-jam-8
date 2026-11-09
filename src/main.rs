use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy::audio::{AudioPlugin, Volume};

mod assets;
mod falling;
mod hand;
mod items;
mod merges;
mod sounds;

const GRID_WIDTH: i32 = 5;
const GRID_HEIGHT: i32 = 8;

const GRID: Vec2 = Vec2::new(GRID_WIDTH as f32, GRID_HEIGHT as f32);
const CELL_SIZE: f32 = 60.0;
const SPRITE_SCALE: f32 = 0.09;

const MOVEMENT_SPEED: f32 = 500.0;
const CLOSE_ENOUGH_DISTANCE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Jam #8 - Alchemy".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resolution: WindowResolution::new(
                            ((GRID_WIDTH + 2) as f32 * CELL_SIZE) as u32,
                            ((GRID_HEIGHT + 4) as f32 * CELL_SIZE) as u32,
                        ),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume::new(Volume::Linear(0.3)),
                    ..default()
                }),
        )
        .init_state::<Phase>()
        .add_systems(Startup, setup.spawn())
        // Choosing
        .add_systems(OnEnter(Phase::Choosing), hand::spawn_hand)
        .add_systems(
            Update,
            (hand::move_hand, hand::rotate_hand, hand::drop_hand)
                .chain()
                .run_if(in_state(Phase::Choosing)),
        )
        .add_systems(Update, (update_world_pos, hand::update_hand_world_pos))
        // Falling
        .add_systems(
            Update,
            (falling::start_falling, falling::items_fall)
                .chain()
                .run_if(in_state(Phase::Falling)),
        )
        // Merging
        .add_systems(OnEnter(Phase::Merging), merges::find_merges)
        .add_systems(
            Update,
            (
                merges::merge_animation
                // , bubbles
                // , shockwave
            )
                .run_if(in_state(Phase::Merging)),
        )
        .init_resource::<assets::AllAssets>()
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum Phase {
    #[default]
    Choosing,
    Falling,
    Merging,
    Lose,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
        Transform::from_translation(Vec3::new(0.0, CELL_SIZE, 0.0)) // swift down the grid
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

fn update_world_pos(positions: Query<(&Position, &mut Transform)>, time: Res<Time>) {
    for (pos, mut transform) in positions {
        let movement = pos.to_world() - transform.translation;
        if movement.length_squared() <= CLOSE_ENOUGH_DISTANCE {
            continue;
        }
        let step = (movement.normalize() * time.delta_secs() * MOVEMENT_SPEED)
            .clamp_length_max(movement.length());
        transform.translation += step;
    }
}

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
