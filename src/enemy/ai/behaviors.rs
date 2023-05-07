use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use super::Ai;

pub fn do_behaviors(mut query: Query<(&mut Ai, &mut Transform)>, time: Res<Time>) {
    for (mut ai, mut transform) in query.iter_mut() {
        let mut info = BehaviorInfo {
            transform: &mut transform,
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
}

impl Behavior {
    pub fn perform(&mut self, info: &mut BehaviorInfo) {
        match self {
            Behavior::Idle => {}
            Behavior::Moving { x, y } => {
                info.transform.translation +=
                    Vec2::new(*x, *y).extend(0.0) * info.time.delta_seconds();
            }
        }
    }
}

pub struct BehaviorInfo<'a> {
    pub transform: &'a mut Transform,
    pub time: &'a Time,
}
