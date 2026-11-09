use bevy::prelude::*;
use rand::RngExt;

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
        .add_systems(OnEnter(Phase::Choosing), spawn_hand.spawn())
        .add_systems(
            Update,
            (move_hand, rotate_hand, drop_hand)
                .chain()
                .run_if(in_state(Phase::Choosing)),
        )
        .add_systems(Update, (update_world_pos, update_hand_world_pos))
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

#[derive(Component, Clone, Copy, PartialEq, Default)]
struct Hand;

#[derive(Component, Clone, Copy, PartialEq, Default)]
struct PositionInHand(i32, i32);

impl PositionInHand {
    fn to_hand_translation(&self) -> Vec3 {
        let PositionInHand(x, y) = *self;
        let pos = Vec3::new(x as f32, y as f32, 0.0);
        pos * CELL_SIZE
    }

    fn to_game_position(&self, hand_pos: &Position) -> Position {
        Position(hand_pos.0 + self.0, hand_pos.1 + self.1)
    }
}

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

fn spawn_hand() -> impl Scene {
    let mut rng = rand::rng();
    let item_a = Item(rng.random_range(0..3));
    let item_b = Item(rng.random_range(0..3));

    let hand_pos = Position(2, GRID_HEIGHT as i32 + 1);
    let hand_pos_a = PositionInHand(0, 0);
    let hand_pos_b = PositionInHand(1, 0);

    bsn! {
        Hand
        hand_pos
        Transform::from_translation(hand_pos.to_world())
        Children [
            item_a
            Text2d::new(item_a.0.to_string())
            hand_pos_a
            Transform::from_translation(hand_pos_a.to_hand_translation())
            --
            item_b
            Text2d::new(item_b.0.to_string())
            hand_pos_b
            Transform::from_translation(hand_pos_b.to_hand_translation())
        ]
    }
}

fn move_hand(
    keys: Res<ButtonInput<KeyCode>>,
    hand: Single<(&mut Position, &Children), With<Hand>>,
    hand_item_pos: Query<&PositionInHand>,
) {
    let dx = keys.just_pressed(KeyCode::ArrowRight) as i32
        - keys.just_pressed(KeyCode::ArrowLeft) as i32;
    if dx == 0 {
        return; // no movement
    }

    let (mut pos, children) = hand.into_inner();
    let next_x = pos.0 + dx;

    let hand_size = children
        .into_iter()
        .flat_map(|child| hand_item_pos.get(*child))
        .map(|pos| pos.0 + 1)
        .max()
        .unwrap_or(1);

    if next_x < 0 || GRID_WIDTH - hand_size < next_x {
        return; // outside
    }
    pos.0 = next_x;
}

fn rotate_hand(
    keys: Res<ButtonInput<KeyCode>>,
    hand: Single<(&mut Position, &Children), With<Hand>>,
    mut hand_item_pos: Query<&mut PositionInHand>,
) {
    if !keys.just_pressed(KeyCode::ArrowUp) {
        return;
    }

    // Rotate around the center of the 2x2 box
    for mut pos in hand_item_pos.iter_mut() {
        let next_pos = match *pos {
            PositionInHand(0, 0) => PositionInHand(1, 0),
            PositionInHand(1, 0) => PositionInHand(1, 1),
            PositionInHand(1, 1) => PositionInHand(0, 1),
            PositionInHand(0, 1) => PositionInHand(0, 0),
            _ => PositionInHand(0, 0),
        };
        pos.0 = next_pos.0;
        pos.1 = next_pos.1;
    }

    let (mut hand_pos, children) = hand.into_inner();

    // Move the hand back if too big
    let hand_size = children
        .into_iter()
        .flat_map(|child| hand_item_pos.get(*child))
        .map(|pos| pos.0 + 1)
        .max()
        .unwrap_or(1);

    if hand_pos.0 > GRID_WIDTH - hand_size {
        hand_pos.0 -= 1;
    }
}

fn drop_hand(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    hand: Single<(Entity, &Position), With<Hand>>,
    hand_item_pos: Query<(Entity, &PositionInHand)>,
) {
    if !keys.just_pressed(KeyCode::ArrowDown) {
        return;
    }
    let (hand_entity, hand_pos) = hand.into_inner();

    // Detach the item from the hand into the game
    for (item_entity, pos_in_hand) in hand_item_pos {
        commands
            .entity(item_entity)
            .remove_parent_in_place()
            .remove::<PositionInHand>()
            .insert(pos_in_hand.to_game_position(hand_pos));
    }

    // Spawn a new hand
    commands.entity(hand_entity).despawn();
    commands.spawn_scene(spawn_hand());
}

const HAND_ANIMATION_SPEED: f32 = 300.0;
const CLOSE_ENOUGH_DISTANCE: f32 = 10.0;

fn update_world_pos(positions: Query<(&Position, &mut Transform)>, time: Res<Time>) {
    for (pos, mut transform) in positions {
        let movement = pos.to_world() - transform.translation;
        if movement.length_squared() <= CLOSE_ENOUGH_DISTANCE {
            continue;
        }
        let direction = movement.normalize() * time.delta_secs() * HAND_ANIMATION_SPEED;
        transform.translation += direction;
    }
}

fn update_hand_world_pos(hand_pos: Query<(&PositionInHand, &mut Transform)>, time: Res<Time>) {
    for (pos, mut transform) in hand_pos {
        let movement = pos.to_hand_translation() - transform.translation;
        if movement.length_squared() <= CLOSE_ENOUGH_DISTANCE {
            continue;
        }
        let direction = movement.normalize() * time.delta_secs() * HAND_ANIMATION_SPEED;
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
