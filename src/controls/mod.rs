use bevy::{input::mouse::MouseMotion, prelude::*, render::view::RenderLayers};
use bevy_mod_raycast::prelude::*;

use crate::{
    solidcomp::SideComponent,
    views::{
        camera_ortho_controller::get_view_mat,
        split::{ActiveSplit, CameraView},
    },
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DeferredRaycastingPlugin::<MyRaycastSet>::default())
            .insert_resource(RaycastPluginState::<MyRaycastSet>::default().with_debug_cursor())
            // .add_systems(Update, (intersection, update_selected))
            .add_systems(Update, print_intersections::<MyRaycastSet>);
        /*
                app
                    // The DefaultRaycastingPlugin bundles all the functionality you might need into a single
                    // plugin. This includes building rays, casting them, and placing a debug cursor at the
                    // intersection. For more advanced uses, you can compose the systems in this plugin however
                    // you need. For example, you might exclude the debug cursor system.
                    .add_plugins(DeferredRaycastingPlugin::<MyRaycastSet>::default())
                    .add_plugins(DeferredRaycastingPlugin::<OrthoRaycastSet>::default())
                    // You will need to pay attention to what order you add systems! Putting them in the wrong
                    // order can result in multiple frames of latency. Ray casting should probably happen near
                    // start of the frame. For example, we want to be sure this system runs before we construct
                    // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
                    // order your systems with the ones provided by the raycasting plugin.
                    // .add_system(
                    //     update_raycast_with_cursor
                    //         .in_base_set(CoreSet::First)
                    //         .before(RaycastSystem::BuildRays::<MyRaycastSet>)
                    //         .before(RaycastSystem::BuildRays::<OrthoRaycastSet>),
                    // )
                    .add_systems(
                        Update,
                        update_raycast_with_cursor
                            .before(RaycastSystem::BuildRays::<MyRaycastSet>)
                            .before(RaycastSystem::BuildRays::<OrthoRaycastSet>),
                    )
                    .add_systems(Update, intersection)
                    .add_systems(Update, ortho_intersection)
                    .add_systems(Update, update_selected);
        */
    }
}

#[derive(Component)]
pub struct Selected(pub bool);

/// Report intersections
fn intersection(
    q_possible_mesh_hits: Query<&Parent, With<RaycastMesh<MyRaycastSet>>>,
    mut q_selected: Query<&mut Selected>,
    source: Query<&RaycastSource<MyRaycastSet>>,
    click: Res<ButtonInput<MouseButton>>,
    space: Res<ButtonInput<KeyCode>>,
    active_split: Res<ActiveSplit>,
) {
    if click.just_pressed(MouseButton::Left)
        && !space.pressed(KeyCode::Space)
        && active_split.is(CameraView::View3D)
    {
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

fn ortho_intersection(
    q_possible_mesh_hits: Query<&Parent, With<RaycastMesh<OrthoRaycastSet>>>,
    mut q_side: Query<(&mut SideComponent, &mut Transform)>,
    sources: Query<&RaycastSource<OrthoRaycastSet>>,
    click: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    space: Res<ButtonInput<KeyCode>>,
    active_split: Res<ActiveSplit>,
) {
    if click.pressed(MouseButton::Left) && !space.pressed(KeyCode::Space) && active_split.is_ortho()
    {
        for source in sources.iter() {
            if let (Some((entity, _)), ActiveSplit::View(view, _)) =
                (source.intersections().get(0), &*active_split)
            {
                if let Ok((mut side, mut trans)) = q_possible_mesh_hits
                    .get(*entity)
                    .and_then(|parent| q_side.get_mut(parent.get()))
                {
                    let mut drag = Vec2::ZERO;
                    for ev in mouse_motion.read() {
                        drag += ev.delta;
                    }
                    trans.translation -= get_view_mat(view) * drag.extend(0.0) * 1.0;
                    dbg!(&trans.translation);
                    trans.translation = (trans.translation / 4.0).round() * 4.0;
                    dbg!(&trans.translation);
                }
            }
        }
    }
}

fn update_selected(
    q_selection_change: Query<(&Children, &Selected), Changed<Selected>>,
    q_3d_view_child: Query<(&Handle<StandardMaterial>, &RenderLayers)>,
    mut q_control_nob: Query<&mut Visibility, With<ControlNob>>,
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

            if let Ok(mut vis) = q_control_nob.get_mut(*child) {
                if selected.0 {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
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
    mut query1: Query<&mut RaycastSource<MyRaycastSet>>,
    mut query2: Query<&mut RaycastSource<OrthoRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.read().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query1 {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
    for mut pick_source in &mut query2 {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

#[derive(Clone, Reflect)]
pub struct OrthoRaycastSet;

#[derive(Component)]
pub struct ControlNob;
