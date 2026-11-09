use bevy::{platform::collections::HashSet, prelude::*};
use rand::seq::IteratorRandom;

use crate::assets::AllAssets;

use super::SPRITE_SCALE;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Natural(usize),
    Mineral(usize),
    Halloween(usize),
    PhilosopherStone,
}

impl Item {
    pub fn random(available_items: &HashSet<Item>) -> Item {
        let mut rng = rand::rng();

        *available_items
            .iter()
            .choose(&mut rng)
            .unwrap_or(&Item::Natural(0))
    }

    pub fn sprite(&self, assets: &AllAssets) -> Handle<Image> {
        match *self {
            Item::Natural(i) => assets.natural_item_sprites[i].clone(),
            Item::Mineral(i) => assets.mineral_item_sprites[i].clone(),
            Item::Halloween(i) => assets.halloween_item_sprites[i].clone(),
            Item::PhilosopherStone => assets.end_item_sprite.clone(),
        }
    }

    pub fn transform(translation: Vec3) -> Transform {
        Transform {
            translation,
            scale: Vec3::splat(SPRITE_SCALE),
            ..default()
        }
    }

    pub fn next_item(&self) -> Option<Item> {
        pub const NB_ITEMS: usize = 3;
        match *self {
            Item::Natural(i) => (i < NB_ITEMS).then_some(Item::Natural(i + 1)),
            Item::Mineral(i) => (i < NB_ITEMS).then_some(Item::Mineral(i + 1)),
            Item::Halloween(i) => (i < NB_ITEMS).then_some(Item::Halloween(i + 1)),
            Item::PhilosopherStone => None,
        }
    }
}
