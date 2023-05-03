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
    geometry::StandardPlane,
    solidcomp::SolidComponent,
    vmf2::{
        res::{ActiveVmf, VmfFile},
        vmf::{self, Point},
    },
};

fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

fn create_lines() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    // mesh.set_indices(None);
    mesh
}

/*
#[derive(Component)]
pub struct Thingy;

pub fn wiggle_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut asdf: Query<&Handle<Mesh>, &Thingy>,
    mut pulse: Local<f32>,
) {
    let handle = asdf.single_mut();
    let start = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    *pulse += 0.1;
    let v = start
        .into_iter()
        .map(|[a, b, c]| {
            [
                (a * *pulse).rem_euclid(5.0),
                (b * *pulse).rem_euclid(5.0),
                (c * *pulse).rem_euclid(5.0),
            ]
        })
        .collect::<Vec<_>>();
    meshes
        .get_mut(handle)
        .unwrap()
        .insert_attribute(Mesh::ATTRIBUTE_POSITION, v);
}
 */
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
                // let points: Vec<[f32;3]>;

                let mut sides: Vec<Vec<Vec3>> = Vec::new();

                for (i, p1) in planes.iter().enumerate() {
                    let mut points = Vec::new();

                    let mut start = None;

                    'outer: for (j, p2) in planes.iter().enumerate() {
                        for (k, p3) in planes.iter().enumerate() {
                            if i != j && j != k && i != k {
                                if let Some(point) = p1.intersection_point(p2, p3) {
                                    points.push(point);
                                    start = Some((j, k));
                                    break 'outer;
                                }
                            }
                        }
                    }

                    if let None = start {
                        break;
                    }

                    let (start_j, start_k) = start.unwrap();
                    let mut j = start_j;
                    let mut k = start_k;

                    fn find_with_with_without(
                        planes: &Vec<StandardPlane>,
                        required: usize,
                        with: usize,
                        without: usize,
                    ) -> Option<(usize, Vec3)> {
                        for (i, plane) in planes.iter().enumerate() {
                            if i != required && i != with && i != without {
                                if let Some(point) =
                                    plane.intersection_point(&planes[required], &planes[with])
                                {
                                    return Some((i, point));
                                }
                            }
                        }
                        return None;
                    }

                    while let Some((index, point)) = find_with_with_without(&planes, i, j, k) {
                        println!("estoy loopin? {k} {j}");
                        points.push(point);
                        k = j;
                        j = index;

                        if j == start_j && k == start_k || j == start_k && k == start_j {
                            break;
                        }
                    }

                    sides.push(points);
                }

                println!("{sides:#?}");
                /*
                               for (i, p1) in planes.iter().enumerate() {
                                   let mut points = Vec::new();

                                   let mut start = None;

                                   'outer: for (j, p2) in planes.iter().enumerate() {
                                       for (k, p3) in planes.iter().enumerate() {
                                           if i != j && j != k && i != k {
                                               if let Some(point) = p1.intersection_point(p2, p3) {
                                                   points.push(point);
                                                   start = Some((j,k));
                                                   break 'outer;
                                               }
                                           }
                                       }
                                   }

                                   match start {
                                       Some(start) => {
                                           let mut last = start;
                                           loop {
                                               for (i, p) in planes.iter().enumerate() {
                                                   if i != last.0 {
                                                       if let Some(point) = p1.intersection_point(p,&planes[last.1]) {
                                                           points.push(point);

                                                           let tmp = last.0;
                                                           last.0 = last.1;
                                                           last.1 = tmp;
                                                       }
                                                   }
                                               }

                                               if last.0 == start.0 && last.1 == start.1 || last.0 == start.1 && last.1 == start.0 {
                                                   break;
                                               }
                                           }
                                       }
                                       None => continue,
                                   }

                                   println!("{points:#?}");
                                   sides.push(points);
                               }
                */
                /*
                for i in 0..planes.len() {
                    let current_plane = &planes[i];

                    // prepare the new side
                    sides.insert(i, Vec::new());
                    let points = &mut sides[i];

                    // find all points that make up this side by looking at all intersections of two other planes with this one
                    for j in 0..planes.len() {
                        if i == j { continue }
                        for k in 0..planes.len() {
                            if j == k || i == k { continue }
                            if let Some(point) = current_plane.intersection_point(&planes[j], &planes[k]) {
                                points.push(point);
                            }
                        }
                    }


                    // We need to sort the points we found on a place so that they are clockwise (or maybe counter clockwise? I don't remember)
                    // in any case, they can't be in a random order, as we only want to draw the outline
                    // currently doesn't work
                    if points.len() > 0 {
                        let t = points.iter().sum::<Vec3>() / points.len() as f32;
                        points.sort_by(|l, r| {
                            let l = (t-*l).project_onto(Vec3::new(1.0,1.0,0.0));
                            let lang = l.x.atan2(l.y);

                            let r = (t-*r).project_onto(Vec3::new(1.0,1.0,0.0));
                            let rang = r.x.atan2(r.y);

                            if lang < rang {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            }
                        });
                    }
                }
                */
                for side in sides {
                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, side.clone());
                    let mut idx = Vec::new();
                    for x in 2..side.len() - 1 {
                        idx.push((x + 1) as u16);
                        idx.push(x as u16);
                        idx.push(1);
                    }
                    mesh.set_indices(Some(Indices::U16(idx)));
                    mesh.duplicate_vertices();
                    mesh.compute_flat_normals();

                    commands.spawn((
                        PbrBundle {
                            // transform: Transform::from_translation(Vec3::new(10_000.0, 9_000.0,0.0)).with_scale(Vec3::ONE * 0.01),
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

                    let mut linemesh = Mesh::new(PrimitiveTopology::LineStrip);
                    linemesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, side);
                    linemesh.set_indices(None);

                    commands.spawn((
                        PbrBundle {
                            // transform: Transform::from_translation(Vec3::new(10_000.0, 9_000.0,0.0)).with_scale(Vec3::ONE * 0.01),
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

                // let Point { x: x1, y: y1, z: z1 } = solid.sides[0].plane.points[0];
                // let Point { x: x2, y: y2, z: z2 } = solid.sides[3].plane.points[0];
                // let b = shape::Box {
                //     min_x: x2,
                //     max_x: x1,
                //     min_y: y2,
                //     max_y: y1,
                //     min_z: z2,
                //     max_z: z1,
                // };

                // println!("{b:#?}");

                // commands.spawn((
                //     PbrBundle {
                //         // transform: Transform::from_translation(Vec3::new(10_000.0, 9_000.0,0.0)).with_scale(Vec3::ONE * 0.01),
                //         transform: Transform::from_scale(Vec3::splat(1.0/128.0)),
                //         mesh: meshes.add(Mesh::from(b)),
                //         material: materials.add(StandardMaterial {
                //             base_color: Color::rgb(1.0, 0.0, 0.0),
                //             unlit: true,
                //             ..default()
                //         }),
                //         ..Default::default()
                //     },
                //     SolidComponent {id: solid.id},
                //     RenderLayers::layer(1),
                // ));
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
                rotation: Quat::from_axis_angle(Vec3::new(0.1, -1.0, 0.3), 0.0),
                // Quat::from_rotation_x(-PI / 8.) * Quat::from_rotation_y(-PI / 3.),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(0),
    ));

    /*
       commands.spawn((
           PbrBundle {
               mesh: meshes.add(Mesh::from(shape::Plane {
                   size: 5.0,
                   subdivisions: 0,
               })),
               material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
               ..Default::default()
           },
           RenderLayers::layer(1),
       ));

       commands.spawn((
           PbrBundle {
               mesh: meshes.add(create_lines()),
               material: materials.add(StandardMaterial {
                   base_color: Color::rgb(1.0, 0.5, 1.0),
                   unlit: true,
                   depth_bias: -10.0,
                   ..default()
               }),
               ..Default::default()
           },
           // MaterialMeshBundle {
           //     mesh: meshes.add(create_triangle()),
           //     material: materials.add(Color::rgb(1.0,0.4,0.2).into()),
           //     ..default()
           // },
           Thingy,
           RenderLayers::layer(0),
       ));
    */
    /*
        commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(4.0, 8.0, 4.0),
                ..Default::default()
            },
            RenderLayers::layer(0),
        ));
    */
}
