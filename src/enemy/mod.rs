mod ai;

pub use ai::*;
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{
    billboard_sprite::BillboardSpriteBundle,
    health::{Health, HealthOptions},
    loader, FromOptions,
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<EnemyOptionsLoader>()
            .add_asset::<EnemyOptions>()
            .add_plugin(AiPlugin)
            .add_system(load_enemies);
    }
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "57422828-c764-11ed-afa1-0242ac120002"]
pub struct EnemyOptions {
    pub health: HealthOptions,
    pub sprite: String,
    pub ai: String,
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
            commands
                .entity(entity)
                .insert((
                    Health::from_options(&options.health),
                    BillboardSpriteBundle::new_anchored(asset_server.load(&options.sprite)),
                ))
                .remove::<Handle<EnemyOptions>>();
        } else {
            println!("this thang taking fookin aages to load init");
        }
    }
}
