use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
    billboard_sprite::{BillboardSprite, BillboardSpriteBundle},
    bullet::{BulletBundle, BulletOptions, Team},
    camera::DiagonalCameraBundle,
    health::Health,
};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_shooting)
            .add_system(player_movement);
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SpatialBundle::default(),
            Player,
            Health::new(100, Team::Player),
            Name::new("Player"),
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
    query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    bullets: Res<Assets<BulletOptions>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // for mut shooting in &mut query {
        //     if let Some(bullet_options) = bullets.get(&asset_server.load("bullet.bullet")) {
        //         shooting.shoot(bullet_options.clone(), PI / 2.0);
        //     }
        // }
        for transform in &query {
            commands.spawn(BulletBundle::new(
                BulletOptions {
                    damage: 40,
                    speed: 3.0,
                    lifetime: 5.0,
                    sprite: "bullet.png".into(),
                    diagonal_sprite: true,
                    team: Team::Player,
                },
                transform.rotation.to_euler(EulerRot::XYZ).1,
                transform.translation.truncate(),
                &asset_server,
            ));
            // if let Some(bullet_options) = bullets.get(&asset_server.load("bullet.bullet")) {
            //     shooting.shoot(bullet_options.clone(), PI / 2.0);
            // }
        }
    }
}
