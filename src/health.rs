use bevy::{prelude::Component, prelude::*, reflect::TypeUuid};
use serde;
use serde::Deserialize;

use crate::bullet::Team;
use crate::FromOptions;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_dead).register_type::<Health>();
    }
}

pub fn despawn_dead(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if health.current == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component, Reflect)]
pub struct Health {
    max: u32,
    current: u32,
    pub team: Team,
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "c2036e7e-c764-11ed-afa1-0242ac120002"]
pub struct HealthOptions {
    pub max: u32,
}

#[allow(dead_code)]
impl Health {
    pub fn new(health: u32, team: Team) -> Self {
        Self {
            max: health,
            current: health,
            team,
        }
    }

    pub fn inflict_damage(&mut self, value: u32) {
        self.current = self.current.saturating_sub(value);
    }

    pub fn heal(&mut self, value: u32) {
        self.current += value;

        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn frac(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

impl FromOptions<HealthOptions> for Health {
    fn from_options(options: &HealthOptions) -> Self {
        Self {
            max: options.max,
            current: options.max,
            team: Team::Enemy,
        }
    }
}
