mod ai;
pub mod drop_table;

use std::fmt::Debug;

pub use ai::*;
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{
    billboard_sprite::{BillboardSpriteBundle, SPRITE8},
    health::Health,
    loader,
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

#[derive(Deserialize, TypeUuid)]
#[uuid = "57422828-c764-11ed-afa1-0242ac120002"]
pub struct EnemyOptions {
    pub health: Health,
    pub sprite: SHandle<Image>,
    pub ai: Ai,
    pub drop_table: DropTable,
}

loader!(EnemyOptions, EnemyOptionsLoader, &["enemy"]);

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
            options.sprite.load(&asset_server);

            commands
                .entity(entity)
                .insert((
                    options.health.clone(),
                    BillboardSpriteBundle::new_anchored(options.sprite.unwrap()),
                    options.drop_table.clone(),
                ))
                .remove::<Handle<EnemyOptions>>();
        }
    }
}

#[derive(Deserialize, TypeUuid, Clone, Reflect, Debug, FromReflect)]
#[uuid = "57422828-c764-11ed-aca1-0242ac120002"]
pub enum SHandle<T: bevy::asset::Asset + Reflect + Debug + FromReflect> {
    Serialized(String),
    #[serde(skip_deserializing)]
    Loaded(Handle<T>),
}

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
}
