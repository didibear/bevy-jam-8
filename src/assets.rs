use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct AllAssets {
    pub natural_item_sprites: Vec<Handle<Image>>,
    pub mineral_item_sprites: Vec<Handle<Image>>,
    pub halloween_item_sprites: Vec<Handle<Image>>,
    pub end_item_sprite: Handle<Image>,
}

impl FromWorld for AllAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            // itch only supports assets directly in the `assets/` directory
            natural_item_sprites: vec![
                asset_server.load("item-water.png"),
                asset_server.load("item-leaf.png"),
                asset_server.load("item-mushroom.png"),
                asset_server.load("item-apple.png"),
            ],
            mineral_item_sprites: vec![
                asset_server.load("item-stone.png"),
                asset_server.load("item-copper.png"),
                asset_server.load("item-gold.png"),
                asset_server.load("item-diamond.png"),
            ],
            halloween_item_sprites: vec![
                asset_server.load("item-candy.png"),
                asset_server.load("item-pumkin-eye.png"),
                asset_server.load("item-bone.png"),
                asset_server.load("item-eye.png"),
            ],
            end_item_sprite: asset_server.load("item-philosophale-stone.png"),
        }
    }
}
