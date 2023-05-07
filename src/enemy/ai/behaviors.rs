use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::player::Player;

use super::Ai;

pub fn do_behaviors(
    mut query: Query<(&mut Ai, &mut Transform), Without<Player>>,
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
) {
    let player_transform = player.single();
    for (mut ai, mut transform) in query.iter_mut() {
        let mut info = BehaviorInfo {
            transform: &mut transform,
            player_transform: &player_transform,
            time: &time,
        };
        ai.do_behaviors(&mut info);
    }
}

#[derive(Component, Deserialize, TypeUuid, Clone)]
#[uuid = "b08c2b7c-a927-46d6-9344-755203047815"]
pub enum Behavior {
    Idle,
    Moving { x: f32, y: f32 },
    ChasePlayer { speed: f32 },
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
        }
    }
}

pub struct BehaviorInfo<'a> {
    pub transform: &'a mut Transform,
    pub time: &'a Time,
    player_transform: &'a Transform,
}
