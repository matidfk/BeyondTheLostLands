pub mod ai;
pub mod drop_table;

pub use ai::*;
use async_trait::async_trait;
use bevy::{
    asset::{AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::texture::CompressedImageFormats,
};
use serde::Deserialize;
use std::{fmt::Debug, path::Path};

use crate::{
    billboard_sprite::{BillboardSpriteBundle, SPRITE8},
    health::Health,
    items::item::Item,
    loader,
    shandle::{load_ron, load_sprite, store_ron, SHandle, SHandleLoad},
};

use self::drop_table::{DropTable, DropTablePlugin};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<EnemyOptionsLoader>()
            .add_asset::<EnemyOptions>()
            .add_plugin(AiPlugin)
            .add_plugin(DropTablePlugin)
            .add_system(load_enemies);
    }
}

#[derive(Deserialize, TypeUuid, Reflect, FromReflect, Debug)]
#[uuid = "57422828-c764-11ed-afa1-0242ac120002"]
pub struct EnemyOptions {
    pub health: Health,
    pub sprite: SHandle<Image>,
    pub ai: Ai,
    pub drop_table: DropTable,
}

#[derive(Default)]
pub struct EnemyOptionsLoader;

impl bevy::asset::AssetLoader for EnemyOptionsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut shandle: SHandle<EnemyOptions> =
                SHandle::Serialized(load_context.path().to_string_lossy().to_string());
            shandle.shandle_load(load_context, true).await?;

            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        &["enemy"]
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub options: Handle<EnemyOptions>,
    pub spatial_bundle: SpatialBundle,
}

pub fn load_enemies(
    mut commands: Commands,
    query: Query<(Entity, &Handle<EnemyOptions>)>,
    mut assets: ResMut<Assets<EnemyOptions>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, handle) in query.iter() {
        if let Some(options) = assets.get_mut(handle) {
            if let SHandle::Loaded(sprite_handle) = &options.sprite {
                commands
                    .entity(entity)
                    .insert((
                        options.health.clone(),
                        BillboardSpriteBundle::new_anchored(sprite_handle.clone()),
                        options.drop_table.clone(),
                        options.ai.clone(),
                    ))
                    .remove::<Handle<EnemyOptions>>();
            } else {
                options.sprite.load(&asset_server);
            }
        }
    }
}
