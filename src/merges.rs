use std::collections::HashMap;

use bevy::{color::palettes::css::*, prelude::*};
use rand::RngExt;

use crate::{CELL_SIZE, Item, Phase, Position};

#[derive(Component, Clone, PartialEq, Default)]
pub struct BeingMerged {
    pub items: Vec<Entity>,
    pub end_pos: Position,
    // next_item_kind: ItemKind,
}

pub fn find_merges(
    mut commands: Commands,
    items: Query<(Entity, &mut Position, &Item), With<Item>>,
) {
    type ItemPos = (Entity, Position, Item);

    fn to_item((ent, pos, it): (Entity, &Position, &Item)) -> ItemPos {
        (ent, *pos, *it)
    }

    let all_items: HashMap<Position, ItemPos> = items.iter().map(|i| (*i.1, to_item(i))).collect();
    let mut todo: Vec<ItemPos> = all_items.values().cloned().collect();

    let items = &all_items;
    let iter_surrounding = |pos: Position| {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .filter_map(move |(i, j)| items.get(&Position(pos.0 + i, pos.1 + j)))
    };

    while let Some(it) = todo.pop() {
        let (_, pos, item) = it;
        let mut items_to_merge: Vec<ItemPos> = vec![it];
        let mut items_to_compare: Vec<&ItemPos> = iter_surrounding(pos).collect();
        let mut seen: Vec<Position> = vec![pos];

        while let Some(&other_it) = items_to_compare.pop() {
            let (_, other_pos, other_item) = other_it;
            seen.push(other_pos);
            if item == other_item {
                items_to_merge.push(other_it);
                for sub_other_it in iter_surrounding(other_pos) {
                    let (_, pos, _) = sub_other_it;
                    if !seen.contains(pos) {
                        items_to_compare.push(sub_other_it);
                    }
                }
            }
        }
        // Consider merged items as done
        todo.retain(|it| items_to_merge.contains(it));

        if items_to_merge.len() < 3 {
            continue;
        }

        let end_pos = *items_to_merge
            .iter()
            .map(|(_, pos, _)| pos)
            .min_by_key(|pos| (pos.1, pos.0)) // bottom then left
            .expect("End position");

        let merged_entities: Vec<Entity> = items_to_merge.iter().map(|(ent, _, _)| *ent).collect();
        for (entity, _, _) in items_to_merge.iter() {
            commands.entity(*entity).insert(BeingMerged {
                items: merged_entities.clone(),
                end_pos,
                // next_item_kind:
            });
        }
    }
}

pub fn merge_animation(
    mut gizmos: Gizmos,
    items: Query<(&Position, &Transform, &BeingMerged)>,
    mut next: ResMut<NextState<Phase>>,
) {
    if items.is_empty() {
        next.set(Phase::Choosing);
    }
    for (pos, tranform, merge) in items {
        let color = if *pos == merge.end_pos { BLUE } else { RED };
        gizmos.circle_2d(tranform.translation.truncate(), CELL_SIZE * 0.5, color);
    }
}
