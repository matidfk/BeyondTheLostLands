use std::fmt::Debug;

use bevy::{
    asset::{LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture::CompressedImageFormats,
};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    bullet::{Bullet, BulletOptions},
    loader,
    shandle::{load_ron, load_sprite, store_ron, SHandle, SHandleLoad},
};

#[derive(TypeUuid, Debug, Reflect, FromReflect, Clone, Deserialize)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325a"]
pub struct Item {
    pub name: String,
    pub sprite: SHandle<Image>,
    pub item_type: ItemType,
}

#[derive(TypeUuid, Debug, Clone, Reflect, FromReflect, Deserialize)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325b"]
pub enum ItemType {
    Regular,
    Equipable(EquipableType),
}

#[derive(TypeUuid, Debug, Clone, Reflect, FromReflect, Deserialize)]
#[uuid = "0635cefa-f22c-4347-8166-38821647325c"]
pub enum EquipableType {
    // path to bullet
    Weapon(SHandle<BulletOptions>),
    Ability,
    Armor,
    Accessory,
}

// loader!(Item, ItemLoader, &["item"]);

#[derive(Default)]
pub struct ItemLoader;

impl bevy::asset::AssetLoader for ItemLoader {
    fn load<'a>(
        &'a self,
        _bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut shandle: SHandle<Item> =
                SHandle::Serialized(load_context.path().to_string_lossy().to_string());
            shandle.shandle_load(load_context, true).await?;

            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["item"]
    }
}
