use bevy::{prelude::Component, prelude::*, reflect::TypeUuid};
use serde;
use serde::Deserialize;

use crate::bullet::Team;
use crate::enemy::drop_table::DropTable;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_dead)
            .register_type::<Health>()
            .add_event::<DeathEvent>();
    }
}

pub struct DeathEvent(pub Entity);

pub fn despawn_dead(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health)>,
    mut ev_death: EventWriter<DeathEvent>,
) {
    for (entity, mut health) in &mut query {
        if health.current == 0 && !health.dead {
            // commands.entity(entity).despawn_recursive();
            ev_death.send(DeathEvent(entity));
            health.dead = true;
        }
    }
}

#[derive(Component, Reflect, Deserialize, TypeUuid, Clone)]
#[uuid = "c2036e7e-c764-11ed-afa1-0242ac120002"]
pub struct Health {
    max: u32,
    current: u32,
    pub team: Team,
    #[serde(skip_deserializing)]
    dead: bool,
}

#[allow(dead_code)]
impl Health {
    pub fn new(health: u32, team: Team) -> Self {
        Self {
            max: health,
            current: health,
            team,
            dead: false,
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
