pub mod inventory;
pub mod item;

use bevy::prelude::*;

use crate::{billboard_sprite::SPRITE8, enemy::SHandle, player::Player};

use self::{
    inventory::Inventory,
    item::{Item, ItemLoader},
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Item>()
            .init_asset_loader::<ItemLoader>()
            // .register_type::<DroppedItem>()
            .add_system(pickup_dropped_items);
    }
}

#[derive(Component)]
pub struct DroppedItem {
    pub item: SHandle<Item>,
}

// #[derive(Bundle)]
// pub struct DroppedItemBundle {
//     item: DroppedItem,
//     sprite_bundle: SpriteBundle,
// }
//
// impl DroppedItemBundle {
//     pub fn new(item: Handle<Item>, position: Vec3, assets: &Assets<Item>) -> Self {
//         Self {
//             sprite_bundle: SpriteBundle {
//                 sprite: SPRITE8,
//                 transform: Transform::from_translation(position),
//                 texture: assets.get(&item).unwrap().sprite.clone(),
//                 ..default()
//             },
//             item: DroppedItem { item },
//         }
//     }
// }

pub fn pickup_dropped_items(
    mut commands: Commands,
    item_query: Query<(Entity, &DroppedItem, &Transform), Without<Player>>,
    mut player: Query<(&Transform, &mut Inventory), With<Player>>,
) {
    let (player_transform, mut inventory) = player.single_mut();

    for (entity, item, transform) in &item_query {
        if transform.translation.distance(player_transform.translation) < 1.0 {
            inventory.contents[3] = Some(item.item.clone());
            commands.entity(entity).despawn();
        }
    }
}
