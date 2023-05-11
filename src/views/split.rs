use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
    window::PrimaryWindow,
};
use bevy_mod_raycast::RaycastSource;

use crate::{
    controls::{MyRaycastSet, OrthoRaycastSet},
    ui::OccupiedScreenSpace,
    views::{
        camera_3d_controller::CameraController, camera_ortho_controller::CameraOrthoController,
    },
};

use super::{
    camera_3d_controller::CameraControllerPlugin,
    camera_ortho_controller::CameraOrthoControllerPlugin,
};

pub struct ChiselCamerasPlugin;

impl Plugin for ChiselCamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cameras)
            .init_resource::<ActiveSplit>()
            .add_system(update_cameras)
            .add_system(update_active_split)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(CameraOrthoControllerPlugin);
    }
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, -6.0).looking_at(Vec3::ZERO, Vec3::Y),
            // transform: Transform::from_xyz(-9_000.0, -1_000.0, -20.0).looking_at(Vec3::new(-9_450.0,-650.0, -24.0), Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        CameraController::default(),
        View3DCamera,
        RenderLayers::layer(0),
        RaycastSource::<MyRaycastSet>::new(),
    ));

    macro_rules! ortho_cam {
        ($comp: ident, $comp2: expr, $order: literal, $transform: expr) => {
            commands.spawn((
                Camera3dBundle {
                    transform: $transform,
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
                $comp2,
                RenderLayers::layer(1),
                RaycastSource::<OrthoRaycastSet>::new(),
            ));
        };
    }

    ortho_cam!(
        TopCamera,
        CameraOrthoController {
            view: CameraView::Top
        },
        1,
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z)
    );
    ortho_cam!(
        FrontCamera,
        CameraOrthoController {
            view: CameraView::Front
        },
        2,
        Transform::from_xyz(100.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
    );
    ortho_cam!(
        SideCamera,
        CameraOrthoController {
            view: CameraView::Side
        },
        3,
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)
    );
}

#[derive(Component)]
pub struct View3DCamera;

#[derive(Component)]
pub struct TopCamera;

#[derive(Component)]
pub struct FrontCamera;

#[derive(Component)]
pub struct SideCamera;

#[derive(Debug, Default, Resource)]
pub enum ActiveSplit {
    #[default]
    None,
    View(CameraView, UVec2),
}

impl ActiveSplit {
    // pub fn is_3d(&self) -> bool {
    //     match self {
    //         ActiveSplit::View(CameraView::View3D, _) => true,
    //         _ => false,
    //     }
    // }

    pub fn is(&self, v: CameraView) -> bool {
        match self {
            ActiveSplit::View(view, _) if v == *view => true,
            _ => false,
        }
    }

    pub fn is_ortho(&self) -> bool {
        self.is(CameraView::Front) || self.is(CameraView::Top) || self.is(CameraView::Side)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CameraView {
    View3D,
    Side,
    Top,
    Front,
}

pub fn update_active_split(
    mut active_split: ResMut<ActiveSplit>,
    occupied_screen_space: Res<OccupiedScreenSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();

    let left = occupied_screen_space.left as u32;
    let right = occupied_screen_space.right as u32;
    let top = occupied_screen_space.top as u32;
    let bottom = occupied_screen_space.bottom as u32;

    let w_height = window.physical_height();

    // Ensure that each viewport has eat least one pixel of width.
    // Zero-width viewports cause a crash (with vulkan at least)
    // (We aren't drawing anything in this function, but we do this to stay consistent)
    let dx = window.physical_width().saturating_sub(left + right).max(2);
    let dy = window.physical_height().saturating_sub(bottom + top).max(2);

    let quarter_x = dx / 2;
    let quarter_y = dy / 2;

    let quarter_size = UVec2::new(quarter_x, quarter_y);

    let view_3d_corner = UVec2::new(left, top);
    let side_corner = view_3d_corner + quarter_size;
    let top_corner = view_3d_corner + UVec2::new(quarter_x, 0);
    let front_corner = view_3d_corner + UVec2::new(0, quarter_y);

    match window.physical_cursor_position() {
        Some(pos) => {
            let pos = UVec2::new(pos.x as u32, w_height.saturating_sub(pos.y as _));

            if pos.cmpge(view_3d_corner).all() && pos.cmple(view_3d_corner + quarter_size).all() {
                *active_split =
                    ActiveSplit::View(CameraView::View3D, view_3d_corner + quarter_size / 2);
            } else if pos.cmpge(top_corner).all() && pos.cmple(top_corner + quarter_size).all() {
                *active_split = ActiveSplit::View(CameraView::Top, top_corner + quarter_size / 2);
            } else if pos.cmpge(front_corner).all() && pos.cmple(front_corner + quarter_size).all()
            {
                *active_split =
                    ActiveSplit::View(CameraView::Front, front_corner + quarter_size / 2);
            } else if pos.cmpge(side_corner).all() && pos.cmple(side_corner + quarter_size).all() {
                *active_split = ActiveSplit::View(CameraView::Side, side_corner + quarter_size / 2);
            } else {
                *active_split = ActiveSplit::None;
            }
            // dbg!(active_split);
        }
        None => *active_split = ActiveSplit::None,
    };
}

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
