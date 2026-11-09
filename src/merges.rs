use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    CLOSE_ENOUGH_DISTANCE, Phase, Position,
    assets::AllAssets,
    items::{self},
    sounds::play_random_sound,
};

#[derive(Component, Clone, PartialEq)]
pub struct BeingMerged {
    pub items: Vec<Entity>,
    pub end_pos: Position,
    pub next_item: Option<items::Item>,
}

pub fn find_merges(
    mut commands: Commands,
    assets: Res<AllAssets>,
    items: Query<(Entity, &mut Position, &items::Item), With<items::Item>>,
    mut next: ResMut<NextState<Phase>>,
) {
    type ItemPos = (Entity, Position, items::Item);

    fn to_item((ent, pos, it): (Entity, &Position, &items::Item)) -> ItemPos {
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
    let mut merge_will_happen = false;

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
        todo.retain(|it| !items_to_merge.contains(it));

        if items_to_merge.len() < 3 {
            continue;
        }
        merge_will_happen = true;

        let end_pos = *items_to_merge
            .iter()
            .map(|(_, pos, _)| pos)
            .min_by_key(|pos| (pos.1, pos.0)) // bottom then left
            .expect("End position");

        let merged_entities: Vec<Entity> = items_to_merge.iter().map(|(ent, _, _)| *ent).collect();
        for (entity, _, item) in items_to_merge.iter() {
            commands
                .entity(*entity)
                .insert(BeingMerged {
                    items: merged_entities.clone(),
                    end_pos,
                    next_item: item.next_item(),
                })
                // Simplify the animation by just going into the position
                .remove::<Position>()
                .insert(end_pos);
        }
    }

    if !merge_will_happen {
        next.set(Phase::Choosing);
    } else {
        commands.spawn(play_random_sound(&assets.merge_sound));
    }
}

pub fn merge_animation(
    mut commands: Commands,
    merging_items: Query<(Entity, &Position, &Transform, &BeingMerged)>,
    mut next: ResMut<NextState<Phase>>,
    assets: Res<AllAssets>,
) {
    // Skip if the find_merges found no merges
    if matches!(
        *next,
        NextState::Pending(Phase::Choosing) | NextState::PendingIfDifferent(Phase::Choosing)
    ) {
        return;
    }

    if merging_items.is_empty() {
        next.set(Phase::Falling);
    }

    let mut groups: HashMap<Position, Vec<_>> = HashMap::new();
    for it in &merging_items {
        groups.entry(it.3.end_pos).or_default().push(it);
    }

    for (_, items) in groups.iter() {
        let all_finished = items.iter().all(|(_, pos, transform, _)| {
            let remaining = pos.to_world() - transform.translation;
            remaining.length_squared() <= CLOSE_ENOUGH_DISTANCE
        });

        if all_finished {
            // Remove old items
            for (entity, _, _, _) in items.iter() {
                commands.entity(*entity).despawn();
            }

            // Spawn new item
            let it = items.iter().next().expect("Item merged");
            let (_, &pos, _, merged) = *it;

            let Some(next_item) = merged.next_item else {
                dbg!("AHNIHILATION");
                continue;
            };

            commands.spawn((
                next_item,
                Sprite::from_image(next_item.sprite(&assets)),
                pos,
                items::Item::transform(pos.to_world()),
            ));
        }
    }
}
