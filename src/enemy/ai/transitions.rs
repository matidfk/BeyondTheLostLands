use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

use crate::health::Health;

use super::Ai;
pub fn do_transitions(mut query: Query<(&mut Ai, &Health)>) {
    for (mut ai, health) in query.iter_mut() {
        let info = TransitionInfo {
            health_frac: health.frac(),
        };
        ai.do_transitions(&info);
    }
}

#[derive(Component, Deserialize, TypeUuid, Clone, Debug, FromReflect, Reflect)]
#[uuid = "b08c2b7c-a927-46d6-9344-755203047814"]
pub enum Transition {
    HealthLessThan(f32),
}

pub struct TransitionInfo {
    pub health_frac: f32,
}

impl Transition {
    pub fn check(&self, info: &TransitionInfo) -> bool {
        match self {
            Transition::HealthLessThan(threshold) => &info.health_frac <= threshold,
        }
    }
}
