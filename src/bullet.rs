use std::f32::consts::PI;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::{billboard_sprite::SPRITE8, enemy::SHandle, health::Health, loader};

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(propagate_bullets)
            .add_system(despawn_bullets)
            .add_system(detect_collisions)
            .add_asset::<BulletOptions>()
            .register_type::<Bullet>()
            .register_type::<BulletOptions>()
            .init_asset_loader::<BulletOptionsLoader>();
    }
}

#[derive(Default, Clone, PartialEq, Reflect, FromReflect, Deserialize, Debug)]
pub enum Team {
    Player,
    #[default]
    Enemy,
}

impl Default for BulletOptions {
    fn default() -> Self {
        Self {
            damage: 40,
            speed: 3.0,
            lifetime: 1.0,
            sprite: SHandle::Serialized("bullet.png".into()),
            team: Team::default(),
            diagonal_sprite: false,
        }
    }
}

#[derive(Component, InspectorOptions, Reflect, Debug)]
pub struct Bullet {
    pub damage: u32,
    pub speed: f32,
    pub direction: f32,
    pub team: Team,
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub sprite_bundle: SpriteBundle,
}

impl BulletBundle {
    pub fn new(
        mut bullet_options: BulletOptions,
        direction: f32,
        position: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let mut sprite_rotation = direction;
        if bullet_options.diagonal_sprite {
            sprite_rotation -= PI / 4.0;
        }

        bullet_options.sprite.load(&asset_server);

        Self {
            sprite_bundle: SpriteBundle {
                texture: bullet_options.sprite.unwrap(),
                transform: Transform::from_translation(position.extend(0.5))
                    .with_rotation(Quat::from_rotation_z(sprite_rotation)),
                sprite: SPRITE8,
                ..default()
            },
            bullet: Bullet {
                damage: bullet_options.damage,
                speed: bullet_options.speed,
                team: bullet_options.team,
                timer: Timer::from_seconds(bullet_options.lifetime, TimerMode::Once),
                direction,
            },
        }
    }
}

pub fn despawn_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in &mut query {
        bullet.timer.tick(time.delta());

        if bullet.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn propagate_bullets(mut query: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut query {
        let mut offset =
            Vec2::new(bullet.direction.cos(), bullet.direction.sin()).extend(0.0) * bullet.speed;

        offset *= time.delta_seconds();
        transform.translation += offset;
    }
}

pub fn detect_collisions(
    mut commands: Commands,
    bullet_query: Query<(&Bullet, &Transform, Entity)>,
    mut health_query: Query<(&mut Health, &Transform)>,
) {
    for (bullet, bullet_transform, bullet_entity) in &bullet_query {
        for (mut health, health_transform) in &mut health_query {
            if bullet_transform
                .translation
                .distance(health_transform.translation)
                < 0.5
                && bullet.team != health.team
            {
                commands.entity(bullet_entity).despawn();
                health.inflict_damage(bullet.damage);
            }
        }
    }
}

//
// impl FromOptions<BulletOptions> for BulletBundle {
//     type Args = (Team, )
//     fn from_options(options: &BulletOptions) -> Self {
//         Self {
//             bullet: Bullet { damage: options.damage, speed: options.speed, direction: , team: (), timer: () },
//             sprite_bundle: todo!(),
//         }
//     }
// }

#[derive(InspectorOptions, Reflect, FromReflect, Deserialize, TypeUuid, Debug, Clone)]
#[reflect(Default)]
#[uuid = "422f5440-c59a-11ed-afa1-0242ac120002"]
pub struct BulletOptions {
    pub damage: u32,
    pub speed: f32,
    pub lifetime: f32,
    pub sprite: SHandle<Image>,
    pub diagonal_sprite: bool,
    pub team: Team,
}

loader!(BulletOptions, BulletOptionsLoader, &["bullet"]);
