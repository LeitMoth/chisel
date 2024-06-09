use std::f32::consts::PI;

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_mod_raycast::prelude::*;

use crate::{
    controls::{ControlNob, OrthoRaycastSet, Selected, View3DRaycastSet},
    geometry::{planes_to_sides, side_to_lines, side_to_triangles, StandardPlane},
    solidcomp::{SideComponent, SolidComponent},
    vmf2::res::{ActiveVmf, VmfFile},
};

pub struct InitPlugin;
impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system)
            .add_systems(Update, change_vmf);
    }
}
pub fn change_vmf(
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
            .and_then(|handle| vmfs_files.get(handle))
        {
            println!("Removing old Solids");
            for solid in &solids {
                commands.entity(solid).despawn_recursive();
            }
            println!("Adding new Solids");
            for solid in &vmf.vmf.world.solids {
                let vmf_sides = &solid.sides;

                let planes: Vec<StandardPlane> = vmf_sides
                    .iter()
                    .map(|s| StandardPlane::new(&s.plane))
                    .collect();

                let sides = planes_to_sides(&planes);

                // println!("{sides:#?}");

                commands
                    .spawn((
                        TransformBundle {
                            // global: GlobalTransform::from_scale(Vec3::splat(1.0 / 128.0)),
                            local: Transform::from_scale(Vec3::splat(1.0 / 128.0)),
                            ..default()
                        },
                        VisibilityBundle::default(),
                        SolidComponent { id: solid.id },
                    ))
                    .with_children(|child_builder| {
                        for side in sides {
                            let avg = side.iter().skip(1).sum::<Vec3>() / (side.len() - 1) as f32;

                            let mesh = side_to_triangles(side.clone());

                            child_builder
                                .spawn((
                                    Selected(false),
                                    SideComponent { id: 5 },
                                    TransformBundle::default(),
                                    VisibilityBundle::default(),
                                ))
                                .with_children(|child_builder| {
                                    child_builder.spawn((
                                        PbrBundle {
                                            // transform: Transform::from_scale(Vec3::splat(1.0 / 128.0)),
                                            mesh: meshes.add(mesh),
                                            material: materials.add(StandardMaterial {
                                                base_color: Color::rgb(1.0, 0.0, 0.0),
                                                double_sided: true,
                                                // cull_mode: None,
                                                cull_mode: None,
                                                perceptual_roughness: 1.0,
                                                reflectance: 0.0,
                                                ..default()
                                            }),
                                            ..Default::default()
                                        },
                                        RenderLayers::layer(0),
                                        NoBackfaceCulling,
                                        RaycastMesh::<View3DRaycastSet>::default(),
                                    ));

                                    let linemesh = side_to_lines(side);
                                    child_builder.spawn((
                                        PbrBundle {
                                            // transform: Transform::from_scale(Vec3::splat(1.0 / 128.0)),
                                            mesh: meshes.add(linemesh),
                                            material: materials.add(StandardMaterial {
                                                base_color: Color::rgb(1.0, 0.0, 0.0),
                                                unlit: true,
                                                ..default()
                                            }),
                                            ..Default::default()
                                        },
                                        RenderLayers::layer(1),
                                    ));

                                    child_builder.spawn((
                                        PbrBundle {
                                            transform: Transform::from_translation(avg),
                                            mesh: meshes.add(Cuboid {
                                                half_size: Vec3::splat(8.0),
                                            }),
                                            material: materials.add(StandardMaterial {
                                                base_color: Color::CYAN,
                                                unlit: true,
                                                ..default()
                                            }),
                                            ..default()
                                        },
                                        ControlNob,
                                        RaycastMesh::<OrthoRaycastSet>::default(),
                                        RenderLayers::layer(1),
                                    ));
                                });
                        }
                    });
            }
        }
    }
}

pub fn setup_system(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
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
