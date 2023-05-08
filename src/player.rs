use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
    billboard_sprite::{BillboardSprite, BillboardSpriteBundle},
    bullet::{BulletBundle, BulletOptions, Team},
    camera::DiagonalCameraBundle,
    health::Health,
    items::{
        inventory::Inventory,
        item::{EquipableType, Item, ItemType},
    },
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_shooting)
            .add_system(inv_debug)
            .add_system(player_movement);
    }
}

pub fn inv_debug(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Inventory, With<Player>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Item>>,
) {
    if keyboard_input.just_pressed(KeyCode::B) {
        let item_handle = crate::enemy::SHandle::Loaded(asset_server.load("weapon.item"));
        query.single_mut().contents[0] = Some(item_handle);
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        if let Some(handle) = &query.single().contents[0] {
            dbg!(handle);
        } else {
            println!("NONE");
        }
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SpatialBundle::default(),
            Player,
            Health::new(100, Team::Player),
            Name::new("Player"),
            Inventory::new(),
        ))
        .with_children(|parent| {
            // camera
            parent.spawn((
                DiagonalCameraBundle {
                    transform: Transform::from_xyz(0.0, -50.0, 50.0)
                        .with_rotation(Quat::from_rotation_x(PI / 4.0)),
                    ..default()
                },
                Name::new("Player Camera"),
            ));

            // sprite
            parent
                .spawn((
                    BillboardSpriteBundle::new_anchored(asset_server.load("character.png")),
                    Name::new("Player Sprite"),
                ))
                // do this for now since billboards dont yet work with children
                .remove::<BillboardSprite>();
        });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    const SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = 4.0;
    let mut movement = Vec2::ZERO;

    let mut rotation = 0.0;

    let mut reset_rotation = false;

    // movement
    if keyboard_input.pressed(KeyCode::W) {
        movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement.x += 1.0;
    }
    // rotation
    if keyboard_input.pressed(KeyCode::Q) {
        rotation += 1.0;
    }
    if keyboard_input.pressed(KeyCode::E) {
        rotation -= 1.0;
    }
    // reset rotation
    if keyboard_input.just_pressed(KeyCode::Z) {
        reset_rotation = true;
    }

    movement = movement.normalize_or_zero();
    movement *= time.delta_seconds();
    movement *= SPEED;

    rotation *= time.raw_delta_seconds();
    rotation *= ROTATION_SPEED;

    for mut transform in &mut query {
        let movement = transform.right() * movement.x + transform.up() * movement.y;
        transform.translation += movement;
        transform.rotate_z(rotation);

        if reset_rotation {
            let new_rotation = -transform.rotation.to_euler(EulerRot::XYZ).2;
            transform.rotate_z(new_rotation);
        }
    }
}

pub fn player_shooting(
    keyboard_input: Res<Input<KeyCode>>,
    _time: Res<Time>,
    mut commands: Commands,
    // mut query: Query<&mut Shooting, With<Player>>,
    query: Query<(&Inventory, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Item>>,
    bullets: Res<Assets<BulletOptions>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (inventory, transform) = query.single();
        if let Some(handle) = &inventory.contents[0] {
            dbg!(handle);
            let item = assets.get_mut(&handle.unwrap()).unwrap();
            if let ItemType::Equipable(equipable) = &mut item.item_type {
                if let EquipableType::Weapon(bullet_handle) = equipable {
                    bullet_handle.load(&asset_server);
                    commands.spawn(BulletBundle::new(
                        bullets.get(&bullet_handle.unwrap()).unwrap().clone(),
                        0.0,
                        transform.translation.truncate(),
                        &asset_server,
                    ));
                }
            }
        }
        // for transform in &query {
        //     commands.spawn(BulletBundle::new(
        //         BulletOptions {
        //             damage: 40,
        //             speed: 3.0,
        //             lifetime: 5.0,
        //             sprite: "bullet.png".into(),
        //             diagonal_sprite: true,
        //             team: Team::Player,
        //         },
        //         transform.rotation.to_euler(EulerRot::XYZ).1,
        //         transform.translation.truncate(),
        //         &asset_server,
        //     ));
        // if let Some(bullet_options) = bullets.get(&asset_server.load("bullet.bullet")) {
        //     shooting.shoot(bullet_options.clone(), PI / 2.0);
        // }
        // }
    }
}
