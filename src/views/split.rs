use bevy::{
    core_pipeline::clear_color::{self, ClearColorConfig},
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
        view::{self, RenderLayers},
    },
    window::PrimaryWindow,
};

use crate::ui::OccupiedScreenSpace;

pub struct ChiselCamerasPlugin;

impl Plugin for ChiselCamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cameras)
            .add_system(update_cameras);
    }
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, -6.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        View3DCamera,
        RenderLayers::layer(0),
    ));

    macro_rules! ortho_cam {
        ($comp: ident, $order: literal) => {
            commands.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
                    camera: Camera {
                        order: $order,
                        ..default()
                    },
                    projection: Projection::Orthographic(OrthographicProjection {
                        scale: 3.0,
                        scaling_mode: ScalingMode::FixedVertical(2.0),
                        ..default()
                    }),
                    camera_3d: Camera3d {
                        // don't clear on the second camera because the first camera already cleared the window
                        clear_color: ClearColorConfig::None,
                        ..default()
                    },
                    ..default()
                },
                $comp,
                RenderLayers::layer(1),
            ));
        };
    }

    ortho_cam!(TopCamera, 1);
    ortho_cam!(FrontCamera, 2);
    ortho_cam!(SideCamera, 3);
}

#[derive(Component)]
pub struct View3DCamera;

#[derive(Component)]
pub struct TopCamera;

#[derive(Component)]
pub struct FrontCamera;

#[derive(Component)]
pub struct SideCamera;

pub fn update_cameras(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
    // mut view_3d_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut camera_set: ParamSet<(
        Query<&mut Camera, With<View3DCamera>>,
        Query<&mut Camera, With<TopCamera>>,
        Query<&mut Camera, With<FrontCamera>>,
        Query<&mut Camera, With<SideCamera>>,
    )>,
) {
    let window = windows.single();

    /*
    Here is what hammer says the views are:
    I think z is up in hammer
    +---------------------------+
    | 3D          | top (x/y)   |
    |---------------------------+
    | front (y/z) | side (x/z)  |
    +---------------------------+
    */

    let left = occupied_screen_space.left as u32;
    let right = occupied_screen_space.right as u32;
    let top = occupied_screen_space.top as u32;
    let bottom = occupied_screen_space.bottom as u32;

    // Ensure that each viewport has eat least one pixel of width.
    // Zero-width viewports cause a crash (with vulkan at least)
    let dx = window.physical_width().saturating_sub(left + right).max(2);
    let dy = window.physical_height().saturating_sub(bottom + top).max(2);

    let quarter_x = dx / 2;
    let quarter_y = dy / 2;

    let quarter_size = UVec2::new(quarter_x, quarter_y);
    let topleft = UVec2::new(left, top);

    let mut view_3d_camera = camera_set.p0();
    let mut view_3d_camera = view_3d_camera.single_mut();
    view_3d_camera.viewport = Some(Viewport {
        physical_position: topleft,
        physical_size: quarter_size,
        ..default()
    });

    let mut top_camera = camera_set.p1();
    let mut top_camera = top_camera.single_mut();
    top_camera.viewport = Some(Viewport {
        physical_position: topleft + UVec2::new(quarter_x, 0),
        physical_size: quarter_size,
        ..default()
    });

    let mut front_camera = camera_set.p2();
    let mut front_camera = front_camera.single_mut();
    front_camera.viewport = Some(Viewport {
        physical_position: topleft + UVec2::new(0, quarter_y),
        physical_size: quarter_size,
        ..default()
    });

    let mut side_camera = camera_set.p3();
    let mut side_camera = side_camera.single_mut();
    side_camera.viewport = Some(Viewport {
        physical_position: topleft + quarter_size,
        physical_size: quarter_size,
        ..default()
    });
}
