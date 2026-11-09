use crate::{
    CELL_SIZE, CLOSE_ENOUGH_DISTANCE, GRID_HEIGHT, GRID_WIDTH, Item, MOVEMENT_SPEED, Phase,
    Position,
};
use bevy::prelude::*;
use rand::RngExt;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct Hand;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct PositionInHand(i32, i32);

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

pub fn spawn_hand() -> impl Scene {
    dbg!("Spawn hand");
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

pub fn move_hand(
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

pub fn rotate_hand(
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

pub fn drop_hand(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    hand: Single<(Entity, &Position), With<Hand>>,
    hand_item_pos: Query<(Entity, &PositionInHand), With<Item>>,
    mut next: ResMut<NextState<Phase>>,
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

    commands.entity(hand_entity).despawn();
    next.set(Phase::Falling);
}

pub fn update_hand_world_pos(hand_pos: Query<(&PositionInHand, &mut Transform)>, time: Res<Time>) {
    for (pos, mut transform) in hand_pos {
        let movement = pos.to_hand_translation() - transform.translation;
        if movement.length_squared() <= CLOSE_ENOUGH_DISTANCE {
            continue;
        }
        let direction = movement.normalize() * time.delta_secs() * MOVEMENT_SPEED;
        transform.translation += direction;
    }
}
