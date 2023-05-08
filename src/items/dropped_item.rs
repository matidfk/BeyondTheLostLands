use bevy::prelude::*;

use crate::{
    billboard_sprite::{BillboardSprite, BillboardSpriteBundle, SPRITE8},
    player::Player,
    shandle::SHandle,
};

use super::{inventory::Inventory, item::Item};
#[derive(Component, Reflect)]
pub struct DroppedItem {
    pub item: SHandle<Item>,
}

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

#[derive(Bundle)]
pub struct DroppedItemBundle {
    dropped_item: DroppedItem,
    sprite_bundle: SpriteBundle,
    billboard_sprite: BillboardSprite,
}

impl DroppedItemBundle {
    pub fn new(item: SHandle<Item>, translation: Vec3, assets: &Assets<Item>) -> Self {
        Self {
            dropped_item: DroppedItem { item: item.clone() },
            sprite_bundle: SpriteBundle {
                sprite: SPRITE8,
                transform: Transform::from_translation(translation),
                texture: assets.get(&item.unwrap()).unwrap().sprite.unwrap(),
                ..default()
            },
            billboard_sprite: BillboardSprite,
        }
    }
}
