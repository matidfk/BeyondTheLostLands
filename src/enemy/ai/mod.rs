mod behaviors;
mod transitions;

use self::{
    behaviors::{do_behaviors, Behavior, BehaviorInfo},
    transitions::{do_transitions, Transition, TransitionInfo},
};
use crate::loader;
use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;
use std::collections::HashMap;

// an ai implementation using a finite state machine.

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(do_behaviors).add_system(do_transitions);
        // .add_system(load_ais)
        // .add_asset::<Ai>()
        // .init_asset_loader::<AiLoader>();
    }
}

pub fn test_ai() -> Ai {
    Ai {
        phases: HashMap::from([
            (
                "Start".into(),
                Phase {
                    behaviors: vec![Behavior::Idle],
                    transitions: vec![(Transition::HealthLessThan(0.5), "2".into())],
                },
            ),
            (
                "2".into(),
                Phase {
                    behaviors: vec![Behavior::Moving { x: 0.5, y: 0.5 }],
                    transitions: vec![],
                },
            ),
        ]),
        current: "Start".into(),
    }
}

// pub fn load_ais(
//     mut commands: Commands,
//     query: Query<(Entity, &Handle<Ai>)>,
//     assets: Res<Assets<Ai>>,
// ) {
//     for (entity, handle) in query.iter() {
//         if let Some(ai) = assets.get(handle) {
//             commands
//                 .entity(entity)
//                 .insert(ai.clone())
//                 .remove::<Handle<Ai>>();
//         } else {
//             println!("this AI thang taking fookin aages to load init");
//         }
//     }
// }

#[derive(Component, Deserialize, TypeUuid, Clone)]
#[uuid = "b08c2b7c-a927-46d6-9344-755203047812"]
pub struct Ai {
    pub phases: HashMap<String, Phase>,
    pub current: String,
}

impl Ai {
    pub fn do_transitions(&mut self, info: &TransitionInfo) {
        for (transition, dest) in self.phases[&self.current].transitions.iter() {
            if transition.check(info) {
                self.current = dest.clone();
            }
        }
    }
    pub fn do_behaviors(&mut self, info: &mut BehaviorInfo) {
        for behavior in self
            .phases
            .get_mut(&self.current)
            .unwrap()
            .behaviors
            .iter_mut()
        {
            behavior.perform(info);
        }
    }
}

#[derive(Component, Deserialize, TypeUuid, Clone)]
#[uuid = "b08c2b7c-a927-46d6-9344-755203047813"]
pub struct Phase {
    pub behaviors: Vec<Behavior>,
    pub transitions: Vec<(Transition, String)>,
}

// loader!(Ai, AiLoader, &["ai"]);
