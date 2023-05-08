use std::time::Duration;

use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::{
    bullet::{BulletBundle, BulletOptions},
    player::Player,
    shandle::SHandle,
};

use super::Ai;

pub fn do_behaviors(
    mut commands: Commands,
    mut query: Query<(&mut Ai, &mut Transform), Without<Player>>,
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    bullet_assets: Res<Assets<BulletOptions>>,
    asset_server: Res<AssetServer>,
) {
    let player_transform = player.single();
    for (mut ai, mut transform) in query.iter_mut() {
        let mut info = BehaviorInfo {
            transform: &mut transform,
            player_transform: &player_transform,
            time: &time,
            commands: &mut commands,
            bullet_assets: &bullet_assets,
            asset_server: &asset_server,
        };
        ai.do_behaviors(&mut info);
    }
}

#[derive(Component, Deserialize, TypeUuid, Clone, Reflect, FromReflect, Debug)]
#[uuid = "b08c2b7c-a927-46d6-9344-755203047815"]
pub enum Behavior {
    Idle,
    Moving {
        x: f32,
        y: f32,
    },
    ChasePlayer {
        speed: f32,
    },
    ShootAtPlayer {
        bullet: SHandle<BulletOptions>,
        interval: f32,
        #[serde(skip_deserializing)]
        timer: Timer,
    },
}

impl Behavior {
    pub fn perform(&mut self, info: &mut BehaviorInfo) {
        match self {
            Behavior::Idle => {}
            Behavior::Moving { x, y } => {
                info.transform.translation +=
                    Vec2::new(*x, *y).extend(0.0) * info.time.delta_seconds();
            }
            Behavior::ChasePlayer { speed } => {
                info.transform.translation += (info.player_transform.translation
                    - info.transform.translation)
                    .normalize_or_zero()
                    * *speed
                    * info.time.delta_seconds();
            }
            Behavior::ShootAtPlayer {
                bullet,
                interval,
                timer,
            } => {
                timer.set_duration(Duration::from_secs_f32(*interval));
                timer.tick(info.time.delta());
                if timer.just_finished() {
                    let bullet_options = info.bullet_assets.get(&bullet.unwrap()).unwrap();
                    info.commands.spawn(BulletBundle::new(
                        bullet_options.clone(),
                        4.0,
                        info.transform.translation.truncate(),
                        info.asset_server,
                    ));
                    timer.reset();
                }
            }
        }
    }
}

pub struct BehaviorInfo<'a, 'w, 's> {
    pub transform: &'a mut Transform,
    pub time: &'a Time,
    pub player_transform: &'a Transform,
    pub commands: &'a mut Commands<'w, 's>,
    pub bullet_assets: &'a Assets<BulletOptions>,
    pub asset_server: &'a AssetServer,
}
