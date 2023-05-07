use crate::{
    billboard_sprite::SPRITE8,
    health::DeathEvent,
    items::{
        item::{Item, ItemOptions},
        DroppedItem,
    },
};
use bevy::{prelude::*, reflect::TypeUuid};
use rand::prelude::*;
use serde::Deserialize;

pub struct DropTablePlugin;

impl Plugin for DropTablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drop_dead_entity_tables)
            .register_type::<DropTable>();
    }
}

#[derive(Component, Debug, Reflect)]
pub struct DropTable {
    pub drops: Vec<(Handle<ItemOptions>, f32)>,
}

impl DropTable {
    pub fn get_items(&self) -> Vec<Handle<ItemOptions>> {
        self.drops
            .iter()
            .filter(|(_item, chance)| thread_rng().gen_bool(*chance as f64))
            .map(|(item, _chance)| item)
            .cloned()
            .collect()
    }

    pub fn from_options(options: &DropTableOptions, asset_server: &AssetServer) -> Self {
        Self {
            drops: options
                .drops
                .iter()
                .map(|(path, chance)| (asset_server.load(path), *chance))
                .collect(),
        }
    }
}

pub fn drop_dead_entity_tables(
    mut commands: Commands,
    query: Query<(&DropTable, &Transform)>,
    mut ev_death: EventReader<DeathEvent>,
    assets: Res<Assets<ItemOptions>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_death.iter() {
        if let Ok((drop_table, transform)) = query.get(ev.0) {
            for item in drop_table.get_items() {
                let item_options = assets.get(&item).unwrap();
                commands.spawn((
                    SpriteBundle {
                        sprite: SPRITE8,
                        transform: Transform::from_translation(transform.translation),
                        texture: asset_server.load(item_options.sprite.clone()),
                        ..default()
                    },
                    DroppedItem { item },
                ));
            }
        }
    }
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "0635cefa-f22c-4347-8166-38831647325c"]
pub struct DropTableOptions {
    pub drops: Vec<(String, f32)>,
}

// #[derive(Deserialize, TypeUuid)]
// #[uuid = "1635cefa-f22c-4347-8166-38831647325c"]
// pub struct NewDropTable {
//     pub drops: Vec<(Jandle<ItemOptions>, f32)>,
// }
