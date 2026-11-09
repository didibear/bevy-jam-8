use crate::{
    CLOSE_ENOUGH_DISTANCE, Phase, Position,
    items::{self},
};
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct ItemFalling;

pub fn start_falling(
    mut commands: Commands,
    items: Query<(Entity, &mut Position), With<items::Item>>,
) {
    let all_pos: Vec<Position> = items.iter().map(|(_, pos)| *pos).collect();

    for (entity, mut pos) in items {
        if pos.1 == 0 {
            continue; // bottom
        }
        if all_pos.contains(&Position(pos.0, pos.1 - 1)) {
            continue; // another item below
        }
        let above_max_y_in_same_col = all_pos
            .iter()
            .filter(|p| p.0 == pos.0 && p.1 < pos.1)
            .map(|p| p.1 + 1)
            .max()
            .unwrap_or(0);

        pos.1 = above_max_y_in_same_col; // destination position
        commands.entity(entity).insert(ItemFalling);
    }
}

pub fn items_fall(
    mut commands: Commands,
    falling_items: Query<(Entity, &Position, &Transform), With<ItemFalling>>,
    mut next: ResMut<NextState<Phase>>,
) {
    if falling_items.is_empty() {
        next.set(Phase::Merging);
    }
    for (entity, pos, transform) in falling_items {
        let remaining = pos.to_world() - transform.translation;
        if remaining.length_squared() <= CLOSE_ENOUGH_DISTANCE {
            commands.entity(entity).remove::<ItemFalling>();
        }
    }
}
