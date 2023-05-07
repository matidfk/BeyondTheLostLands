use std::f32::consts::{PI, SQRT_2};

use bevy::core_pipeline::core_2d::graph;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{Camera, CameraProjection, CameraProjectionPlugin, CameraRenderGraph};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct DiagonalProjection {
    near: f32,
    far: f32,
    aspect: f32,
    scale: f32,
}

impl CameraProjection for DiagonalProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -self.aspect * self.scale * SQRT_2,
            self.aspect * self.scale * SQRT_2,
            -1.0 * self.scale,
            1.0 * self.scale,
            self.near,
            self.far,
        )
    }

    // what to do on window resize
    fn update(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for DiagonalProjection {
    fn default() -> Self {
        Self {
            near: 0.0,
            far: 1000.0,
            aspect: 1.0,
            scale: 5.0,
        }
    }
}

#[derive(Bundle)]
pub struct DiagonalCameraBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: DiagonalProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
    pub tonemapping: Tonemapping,
}

impl Default for DiagonalCameraBundle {
    fn default() -> Self {
        let projection = DiagonalProjection::default();

        let transform = Transform::from_rotation(Quat::from_rotation_x(PI / 4.0));

        // frustum construction code copied from Bevy
        let view_projection =
            projection.get_projection_matrix() * transform.compute_matrix().inverse();

        // let frustum = Frustum::from_view_projection(&view_projection);
        let frustum = Frustum::from_view_projection_custom_far(
            &view_projection,
            &transform.translation,
            &transform.back(),
            projection.far,
        );

        let camera_render_graph = CameraRenderGraph::new(graph::NAME);

        Self {
            camera_render_graph,
            projection,
            frustum,
            transform,
            visible_entities: VisibleEntities::default(),
            global_transform: GlobalTransform::default(),
            camera: Camera::default(),
            camera_2d: Camera2d::default(),
            tonemapping: Tonemapping::None,
        }
    }
}

// hack to get sprite sorting based on screen y position
// TODO: make this better aka not completely overwrite the z position somehow
fn sort_y(
    mut query: Query<(&mut Transform, &GlobalTransform), With<Sprite>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    for (mut transform, global_transform) in query.iter_mut() {
        if let Some(ndc) = camera.world_to_ndc(
            camera_transform,
            global_transform.to_scale_rotation_translation().2,
        ) {
            transform.translation.z = ndc.y * -0.001;
        }
    }
}

pub struct DiagonalProjectionPlugin;

impl Plugin for DiagonalProjectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraProjectionPlugin::<DiagonalProjection>::default())
            .add_system(sort_y);
    }
}
