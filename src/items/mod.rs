pub mod dropped_item;
pub mod inventory;
pub mod item;

use bevy::prelude::*;

use self::{
    dropped_item::{pickup_dropped_items, DroppedItem},
    inventory::Inventory,
    item::{Item, ItemLoader},
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Item>()
            .register_asset_reflect::<Item>()
            .init_asset_loader::<ItemLoader>()
            .register_type::<DroppedItem>()
            .register_type::<Inventory>()
            .add_system(pickup_dropped_items);
    }
}
