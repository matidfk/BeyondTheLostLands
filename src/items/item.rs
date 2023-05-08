use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{
    bullet::{Bullet, BulletOptions},
    enemy::SHandle,
    loader,
};

#[derive(TypeUuid, Debug, Reflect, FromReflect, Clone, Deserialize)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325a"]
pub struct Item {
    pub name: String,
    pub sprite: SHandle<Image>,
    pub item_type: ItemType,
}

#[derive(Deserialize, TypeUuid, Debug, Clone, Reflect, FromReflect)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325b"]
pub enum ItemType {
    Regular,
    Equipable(EquipableType),
}

#[derive(Deserialize, TypeUuid, Debug, Clone, Reflect, FromReflect)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325c"]
pub enum EquipableType {
    // Weapon(AssetPath<BulletOptions>),
    Weapon(SHandle<BulletOptions>),
    Ability,
    Armor,
    Accessory,
}

// #[derive(Deserialize, TypeUuid, Debug, Clone, Reflect)]
// #[uuid = "0625cefa-f22c-4347-8166-38821647325a"]
// pub struct ItemOptions {
//     pub name: String,
//     pub sprite: String,
//     pub item_type: ItemType,
// }
//
loader!(Item, ItemLoader, &["item"]);
