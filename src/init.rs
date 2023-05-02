use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology, view::RenderLayers},
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

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));

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
