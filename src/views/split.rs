use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::Viewport,
    window::PrimaryWindow,
};

use crate::ui::OccupiedScreenSpace;

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Left Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        LeftCamera,
    ));

    // Right Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RightCamera,
    ));
}

#[derive(Component)]
pub struct LeftCamera;

#[derive(Component)]
pub struct RightCamera;

pub fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut left_camera: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
    mut right_camera: Query<&mut Camera, With<RightCamera>>,
) {
    let window = windows.single();

    /*
    3D      top (x/y)

    front (y/z) side (x/z)
    
    
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
    let topleft = UVec2::new(left,top);

    let mut left_camera = left_camera.single_mut();
    left_camera.viewport = Some(Viewport {
        physical_position: topleft,
        physical_size: quarter_size,
        ..default()
    });

    let mut right_camera = right_camera.single_mut();
    right_camera.viewport = Some(Viewport {
        physical_position: topleft + quarter_size,
        physical_size: quarter_size,
        ..default()
    });
}
