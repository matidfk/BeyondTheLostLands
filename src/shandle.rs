use async_trait::async_trait;
use bevy::{
    asset::{LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture::CompressedImageFormats,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{fmt::Debug, path::Path};

use crate::{
    bullet::BulletOptions,
    enemy::{behaviors::Behavior, EnemyOptions},
    items::item::{EquipableType, Item, ItemType},
};

// Serializable handle
#[derive(Deserialize, TypeUuid, Clone, Reflect, Debug, FromReflect)]
#[uuid = "57422828-c764-11ed-aca1-0242ac120002"]
pub enum SHandle<T: bevy::asset::Asset + Reflect + Debug + FromReflect> {
    Serialized(String),
    #[serde(skip_deserializing)]
    Loaded(Handle<T>),
}

// trait to eventually make a shandle load derive macro
//
#[async_trait]
pub trait SHandleLoad
where
    Self: FromReflect + bevy::asset::Asset + Debug + DeserializeOwned,
{
    async fn shandle_load<'a>(
        &mut self,
        load_context: &mut LoadContext<'a>,
        root: bool,
    ) -> Result<(), bevy::asset::Error>;
}

#[async_trait]
impl SHandleLoad for SHandle<Image> {
    async fn shandle_load<'a>(
        &mut self,
        load_context: &mut LoadContext<'a>,
        root: bool,
    ) -> Result<(), bevy::asset::Error> {
        let path = self.path();
        let bytes = load_context.read_asset_bytes(path.clone()).await?;

        let ext = Path::new(&path).extension().unwrap().to_str().unwrap();
        let sprite = Image::from_buffer(
            &bytes,
            bevy::render::texture::ImageType::Extension(ext),
            CompressedImageFormats::all(),
            true,
        )?;

        let handle = load_context.set_labeled_asset(&path, LoadedAsset::new(sprite));
        *self = SHandle::Loaded(handle);
        Ok(())
    }
}

#[async_trait]
impl SHandleLoad for SHandle<Item> {
    async fn shandle_load<'a>(
        &mut self,
        load_context: &mut LoadContext<'a>,
        root: bool,
    ) -> Result<(), bevy::asset::Error> {
        let mut asset = load_ron(self, load_context).await?;
        asset.sprite.shandle_load(load_context, false).await?;
        match &mut asset.item_type {
            ItemType::Regular => {}
            ItemType::Equipable(e) => match e {
                EquipableType::Weapon(bullet_options_shandle) => {
                    bullet_options_shandle
                        .shandle_load(load_context, false)
                        .await?;
                }
                _ => {}
            },
        }
        store_ron(self, asset, load_context, root);

        Ok(())
    }
}

#[async_trait]
impl SHandleLoad for SHandle<BulletOptions> {
    async fn shandle_load<'a>(
        &mut self,
        load_context: &mut LoadContext<'a>,
        root: bool,
    ) -> Result<(), bevy::asset::Error> {
        let mut asset = load_ron(self, load_context).await?;
        asset.sprite.shandle_load(load_context, false).await?;
        store_ron(self, asset, load_context, root);

        Ok(())
    }
}

#[async_trait]
impl SHandleLoad for SHandle<EnemyOptions> {
    async fn shandle_load<'a>(
        &mut self,
        load_context: &mut LoadContext<'a>,
        root: bool,
    ) -> Result<(), bevy::asset::Error> {
        let mut asset = load_ron(self, load_context).await?;
        asset.sprite.shandle_load(load_context, false).await?;

        for (item_shandle, _) in &mut asset.drop_table.drops {
            item_shandle.shandle_load(load_context, false).await?;
        }

        for (_name, phase) in &mut asset.ai.phases {
            for b in &mut phase.behaviors {
                match b {
                    Behavior::ShootAtPlayer {
                        bullet,
                        interval,
                        timer,
                    } => {
                        bullet.shandle_load(load_context, false).await?;
                    }
                    _ => {}
                }
            }
            // item_shandle.shandle_load(load_context, false).await?;
        }

        store_ron(self, asset, load_context, root);

        Ok(())
    }
}

// more direct trait maybe in the future
// impl Load for EnemyOptions {
//      pub fn load(bytes, ctx) {
//          let asset = from_bytes(bytes);
//          // asset.sprite: SHandle<Image>
//          asset.sprite.shandle_load(ctx);
//          asset.drop_table.shandle_load(ctx);
//      }
// }

impl<T: bevy::asset::Asset + Reflect + Debug + FromReflect> SHandle<T> {
    pub fn load(&mut self, asset_server: &AssetServer) {
        if let SHandle::Serialized(path) = self {
            *self = SHandle::Loaded(asset_server.load(path.clone()));
        }
    }

    pub fn unwrap(&self) -> Handle<T> {
        match self {
            SHandle::Serialized(_) => panic!("SHandle not loaded!"),
            SHandle::Loaded(handle) => handle.clone(),
        }
    }

    pub fn path(&self) -> String {
        match self {
            SHandle::Serialized(path) => path.clone(),
            SHandle::Loaded(_) => panic!(),
        }
    }
}

pub async fn load_ron<'a, T: bevy::asset::Asset + Debug + FromReflect + DeserializeOwned>(
    shandle: &mut SHandle<T>,
    load_context: &mut LoadContext<'a>,
) -> Result<T, bevy::asset::Error> {
    let path = shandle.path();
    let bytes = load_context.read_asset_bytes(path.clone()).await?;
    let asset = ron::de::from_bytes::<T>(&bytes)?;
    Ok(asset)
}

pub fn store_ron<'a, T: bevy::asset::Asset + Debug + FromReflect + DeserializeOwned>(
    shandle: &mut SHandle<T>,
    asset: T,
    load_context: &mut LoadContext<'a>,
    root: bool,
) {
    if root {
        load_context.set_default_asset(LoadedAsset::new(asset));
    } else {
        let handle = load_context.set_labeled_asset(&shandle.path(), LoadedAsset::new(asset));
        *shandle = SHandle::Loaded(handle);
    }
}

pub async fn load_sprite<'a>(
    shandle: &mut SHandle<Image>,
    load_context: &mut LoadContext<'a>,
) -> Result<(), bevy::asset::Error> {
    let path = shandle.path();
    let bytes = load_context.read_asset_bytes(path.clone()).await?;
    let sprite = Image::from_buffer(
        &bytes,
        bevy::render::texture::ImageType::Extension("png"),
        CompressedImageFormats::all(),
        true,
    )?;

    let handle = load_context.set_labeled_asset(&path, LoadedAsset::new(sprite));
    *shandle = SHandle::Loaded(handle);
    Ok(())
}
