use bevy::prelude::*;

use crate::enemy::SHandle;

use super::item::Item;

#[derive(Component)]
pub struct Inventory {
    pub contents: [Option<SHandle<Item>>; 8],
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            contents: std::array::from_fn(|_| None),
        }
    }
}
