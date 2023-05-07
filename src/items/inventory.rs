use bevy::prelude::*;

use super::item::{Item, ItemOptions};

#[derive(Component, Debug)]
pub struct Inventory {
    pub contents: [Option<Handle<ItemOptions>>; 8],
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            contents: std::array::from_fn(|_| None),
        }
    }
}
