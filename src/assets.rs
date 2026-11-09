use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct AllAssets {
    pub natural_item_sprites: Vec<Handle<Image>>,
    pub mineral_item_sprites: Vec<Handle<Image>>,
    pub halloween_item_sprites: Vec<Handle<Image>>,
    pub end_item_sprite: Handle<Image>,

    pub drop_sound: Vec<Handle<AudioSource>>,
    pub merge_sound: Vec<Handle<AudioSource>>,
    pub win_sound: Handle<AudioSource>,
    pub lose_sound: Handle<AudioSource>,
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

            // Sounds
            drop_sound: vec![
                asset_server.load("drop1.ogg"),
                asset_server.load("drop2.ogg"),
                asset_server.load("drop3.ogg"),
                asset_server.load("drop4.ogg"),
                asset_server.load("drop5.ogg"),
                asset_server.load("drop6.ogg"),
            ],
            merge_sound: vec![
                asset_server.load("merge1.ogg"),
                asset_server.load("merge2.ogg"),
                asset_server.load("merge3.ogg"),
                asset_server.load("merge4.ogg"),
                asset_server.load("merge5.ogg"),
            ],
            lose_sound: asset_server.load("lose.ogg"),
            win_sound: asset_server.load("win.ogg"),
        }
    }
}
