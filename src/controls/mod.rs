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
        app.add_plugins(DeferredRaycastingPlugin::<View3DRaycastSet>::default())
            .add_plugins(DeferredRaycastingPlugin::<OrthoRaycastSet>::default())
            .insert_resource(RaycastPluginState::<View3DRaycastSet>::default())
            .insert_resource(RaycastPluginState::<OrthoRaycastSet>::default())
            .add_systems(Update, (intersection, ortho_intersection, update_selected));
    }
}

#[derive(Component)]
pub struct Selected(pub bool);

/// Report intersections
fn intersection(
    q_possible_mesh_hits: Query<&Parent, With<RaycastMesh<View3DRaycastSet>>>,
    mut q_selected: Query<&mut Selected>,
    source: Query<&RaycastSource<View3DRaycastSet>>,
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
                if let Ok((_side, mut trans)) = q_possible_mesh_hits
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
pub struct View3DRaycastSet;

#[derive(Clone, Reflect)]
pub struct OrthoRaycastSet;

#[derive(Component)]
pub struct ControlNob;
