use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};

use crate::player::Player;

pub const SPRITE8: Sprite = Sprite {
    custom_size: Some(Vec2::ONE),
    color: Color::WHITE,
    flip_x: false,
    flip_y: false,
    rect: None,
    anchor: Anchor::Center,
};

pub const SPRITE8_ANCHORED: Sprite = Sprite {
    custom_size: Some(Vec2::ONE),
    color: Color::WHITE,
    flip_x: false,
    flip_y: false,
    rect: None,
    anchor: Anchor::BottomCenter,
};

#[derive(Component, Default)]
pub struct BillboardSprite;

#[derive(Bundle, Default)]
pub struct BillboardSpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,

    pub billboard_sprite: BillboardSprite,
}

impl BillboardSpriteBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            sprite: SPRITE8,
            texture,
            transform: Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
            ..default()
        }
    }
    pub fn new_anchored(texture: Handle<Image>) -> Self {
        Self {
            sprite: SPRITE8_ANCHORED,
            ..Self::new(texture)
        }
    }
}

/// Align billboard sprites' y rotation so that they face the player camera
/// TODO: subtract global transform's rotation to work with child objects
pub fn rotate_billboard_sprites(
    mut query: Query<(&mut Transform, &GlobalTransform), (With<BillboardSprite>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let angle = player.single().rotation.to_euler(EulerRot::XYZ).2;
    for (mut transform, _global_transform) in query.iter_mut() {
        transform.rotation = Quat::from_euler(EulerRot::XYZ, PI / 2.0, angle, 0.0);
    }
}

pub struct BillboardSpritePlugin;

impl Plugin for BillboardSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rotate_billboard_sprites);
    }
}
