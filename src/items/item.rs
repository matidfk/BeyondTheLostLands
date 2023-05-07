use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{bullet::BulletOptions, loader};

#[derive(TypeUuid, Debug)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325a"]
pub struct Item {
    pub name: String,
    pub sprite: Handle<Image>,
    pub item_type: ItemType,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325b"]
pub enum ItemType {
    Regular,
    Equipable(EquipableType),
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325c"]
pub enum EquipableType {
    // Weapon(AssetPath<BulletOptions>),
    Weapon(String),
    Ability,
    Armor,
    Accessory,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "0625cefa-f22c-4347-8166-38821647325a"]
pub struct ItemOptions {
    pub name: String,
    pub sprite: String,
    pub item_type: ItemType,
}

loader!(ItemOptions, ItemOptionsLoader, &["item"]);
