mod ai;
pub mod drop_table;

pub use ai::*;
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{
    billboard_sprite::BillboardSpriteBundle,
    health::{Health, HealthOptions},
    items::item::ItemOptions,
    loader, FromOptions,
};

use self::drop_table::{DropTable, DropTableOptions, DropTablePlugin};

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
    pub health: HealthOptions,
    pub sprite: String,
    pub ai: Ai,
    pub drop_table: DropTableOptions,
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
    assets: Res<Assets<EnemyOptions>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, handle) in query.iter() {
        if let Some(options) = assets.get(handle) {
            Box::leak(Box::new(asset_server.load::<ItemOptions, _>("test.item")));
            commands
                .entity(entity)
                .insert((
                    Health::from_options(&options.health),
                    BillboardSpriteBundle::new_anchored(asset_server.load(&options.sprite)),
                    DropTable::from_options(&options.drop_table, &asset_server),
                ))
                .remove::<Handle<EnemyOptions>>();
        } else {
            // println!("loading enemy...")
        }
    }
}

pub struct NewEnemyOptions {
    pub health: HealthOptions,
    pub sprite: SHandle<Image>,
    pub ai: Ai,
    pub drop_table: DropTableOptions,
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "57422828-c764-11ed-aca1-0242ac120002"]
pub enum SHandle<T: bevy::asset::Asset> {
    Serialized(String),
    #[serde(skip_deserializing)]
    Loaded(Handle<T>),
}

impl<T: bevy::asset::Asset> SHandle<T> {
    pub fn load(mut self, asset_server: &AssetServer) {
        if let SHandle::Serialized(path) = self {
            self = SHandle::Loaded(asset_server.load(path));
        }
    }
}
