mod billboard_sprite;
mod bullet;
mod camera;
mod enemy;
mod health;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use billboard_sprite::BillboardSpritePlugin;
use bullet::BulletPlugin;
use camera::DiagonalProjectionPlugin;
use enemy::{Ai, EnemyBundle, EnemyPlugin};
use health::HealthPlugin;
use player::PlayerPlugin;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(BillboardSpritePlugin)
        .add_plugin(DiagonalProjectionPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load::<Ai, _>("test.ai");
    Box::leak(Box::new(handle.clone()));

    commands.spawn((
        EnemyBundle {
            options: asset_server.load("test.enemy"),
            spatial_bundle: default(),
        },
        Name::new("TEST ENTITY"),
        handle,
    ));
}

pub trait FromOptions<O> {
    // type Args;
    // fn from_options(options: &O, args: Self::Args) -> Self;
    fn from_options(options: &O) -> Self;
}

// macro to implement an asset loader
// TODO: move somewhere lol
#[macro_export]
macro_rules! loader {
    ($T:ident, $LOADER:ident, $extensions:expr) => {
        #[derive(Default)]
        struct $LOADER;

        impl bevy::asset::AssetLoader for $LOADER {
            fn load<'a>(
                &'a self,
                bytes: &'a [u8],
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
                Box::pin(async move {
                    let custom_asset = ron::de::from_bytes::<$T>(bytes)?;
                    load_context.set_default_asset(bevy::asset::LoadedAsset::new(custom_asset));
                    Ok(())
                })
            }

            fn extensions(&self) -> &[&str] {
                $extensions
            }
        }
    };
}
