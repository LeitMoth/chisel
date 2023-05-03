use std::{cmp::Ordering, f32::consts::PI, mem::swap};

use bevy::{
    ecs::{system::EntityCommands, world},
    pbr::CascadeShadowConfigBuilder,
    prelude::{shape::Plane, *},
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::PrimitiveTopology,
        view::RenderLayers,
    },
    utils::tracing::span::Attributes,
};

use crate::{
    geometry::{planes_to_sides, side_to_lines, side_to_triangles, StandardPlane},
    solidcomp::SolidComponent,
    vmf2::{
        res::{ActiveVmf, VmfFile},
        vmf::{self, Point},
    },
};

pub fn change(
    active_vmf: Res<ActiveVmf>,
    vmfs_files: Res<Assets<VmfFile>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    solids: Query<Entity, With<SolidComponent>>,
) {
    if active_vmf.is_changed() {
        if let Some(vmf) = active_vmf
            .active
            .as_ref()
            .and_then(|handle| vmfs_files.get(&handle))
        {
            println!("Removing old Solids");
            for solid in &solids {
                commands.entity(solid).despawn();
            }
            println!("Adding new Solids");
            for solid in &vmf.vmf.world.solids {
                let planes: Vec<StandardPlane> = solid
                    .sides
                    .iter()
                    .map(|s| StandardPlane::new(&s.plane))
                    .collect();

                let sides = planes_to_sides(&planes);

                // println!("{sides:#?}");

                for side in sides {
                    let mesh = side_to_triangles(side.clone());

                    commands.spawn((
                        PbrBundle {
                            transform: Transform::from_scale(Vec3::splat(1.0 / 128.0)),
                            mesh: meshes.add(mesh),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 0.0, 0.0),
                                double_sided: true,
                                cull_mode: None,
                                perceptual_roughness: 1.0,
                                reflectance: 0.0,
                                ..default()
                            }),
                            ..Default::default()
                        },
                        SolidComponent { id: solid.id },
                        RenderLayers::layer(0),
                    ));

                    let linemesh = side_to_lines(side);
                    commands.spawn((
                        PbrBundle {
                            transform: Transform::from_scale(Vec3::splat(1.0 / 128.0)),
                            mesh: meshes.add(linemesh),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 0.0, 0.0),
                                unlit: true,
                                ..default()
                            }),
                            ..Default::default()
                        },
                        SolidComponent { id: solid.id },
                        RenderLayers::layer(1),
                    ));
                }
            }
        }
    }
}

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // directional 'sun' light
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 10.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 8.) * Quat::from_rotation_y(-PI / 3.),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(0),
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 10.0, 0.0),
                rotation: Quat::from_rotation_x(PI / 8.) * Quat::from_rotation_y(PI / 4.),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(0),
    ));
}
