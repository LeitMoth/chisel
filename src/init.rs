use std::f32::consts::PI;

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, Intersection, NoBackfaceCulling, RaycastMesh,
    RaycastMethod, RaycastSource, RaycastSystem,
};

use crate::{
    geometry::{planes_to_sides, side_to_lines, side_to_triangles, StandardPlane},
    solidcomp::SolidComponent,
    vmf2::res::{ActiveVmf, VmfFile},
};

pub struct InitPlugin;
impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app
            // The DefaultRaycastingPlugin bundles all the functionality you might need into a single
            // plugin. This includes building rays, casting them, and placing a debug cursor at the
            // intersection. For more advanced uses, you can compose the systems in this plugin however
            // you need. For example, you might exclude the debug cursor system.
            .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
            // You will need to pay attention to what order you add systems! Putting them in the wrong
            // order can result in multiple frames of latency. Ray casting should probably happen near
            // start of the frame. For example, we want to be sure this system runs before we construct
            // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
            // order your systems with the ones provided by the raycasting plugin.
            .add_system(
                update_raycast_with_cursor
                    .in_base_set(CoreSet::First)
                    .before(RaycastSystem::BuildRays::<MyRaycastSet>),
            )
            .add_system(intersection)
            .add_startup_system(setup_system)
            .add_system(update_selected)
            .add_system(change_vmf);
    }
}

#[derive(Component)]
struct Selected(bool);

/// Report intersections
fn intersection(
    q_possible_mesh_hits: Query<&Parent, With<RaycastMesh<MyRaycastSet>>>,
    mut q_selected: Query<&mut Selected>,
    source: Query<&RaycastSource<MyRaycastSet>>,
    click: Res<Input<MouseButton>>,
    space: Res<Input<KeyCode>>,
) {
    if click.just_pressed(MouseButton::Left) && !space.pressed(KeyCode::Space) {
        let source = source.single();

        // Get the first intersection
        if let Some((entity, _)) = source.intersections().get(0) {
            // deselect all
            for mut selected in q_selected.iter_mut() {
                if selected.0 == true {
                    selected.0 = false;
                }
            }

            // find the mesh we clicked on, go to it's parent, which should have a Selected component, then set selected to true.
            if let Ok(mut selected) = q_possible_mesh_hits
                .get(*entity)
                .and_then(|parent| q_selected.get_mut(parent.get()))
            {
                selected.0 = true;
            } else {
                warn!("Clicked on solid that doesn't exist?");
            }
        }
    }
}

fn update_selected(
    q_selection_change: Query<(&Children, &Selected), Changed<Selected>>,
    q_3d_view_child: Query<(&Handle<StandardMaterial>, &RenderLayers)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Look through every entity that had a selection change
    for (children, selected) in q_selection_change.iter() {
        // Look through each child (there should only ever be two but whatever)
        for child in children {
            // If we have both a material and a render layer (should be both children at this point)
            if let Ok((mat_handle, render_layer)) = q_3d_view_child.get(*child) {
                // If we are on the correct render layer (okay we know for sure we are changing the 3d view mesh)
                if render_layer.intersects(&RenderLayers::layer(0)) {
                    materials.get_mut(mat_handle).unwrap().base_color = if selected.0 {
                        Color::YELLOW
                    } else {
                        Color::RED
                    };
                }
            }
        }
    }
}

/// This is a unit struct we will use to mark our generic `RaycastMesh`s and `RaycastSource` as part
/// of the same group, or "RaycastSet". For more complex use cases, you might use this to associate
/// some meshes with one ray casting source, and other meshes with a different ray casting source."
#[derive(Clone, Reflect)]
pub struct MyRaycastSet;

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
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

                    commands
                        .spawn((
                            Selected(false),
                            SolidComponent { id: solid.id },
                            TransformBundle::default(),
                            VisibilityBundle::default(),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                PbrBundle {
                                    transform: Transform::from_scale(Vec3::splat(1.0 / 128.0)),
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
                                RaycastMesh::<MyRaycastSet>::default(),
                            ));

                            let linemesh = side_to_lines(side);
                            parent.spawn((
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
                                RenderLayers::layer(1),
                            ));
                        });
                }
            }
        }
    }
}

pub fn setup_system(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
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
