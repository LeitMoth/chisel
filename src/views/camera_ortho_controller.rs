use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    math::DVec2,
    prelude::*,
    window::CursorGrabMode,
};

use super::split::{ActiveSplit, CameraView};

use bevy::render::camera::Projection::Orthographic;

pub struct CameraOrthoControllerPlugin;

impl Plugin for CameraOrthoControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_ortho_controller);
    }
}

#[derive(Component)]
pub struct CameraOrthoController {
    pub view: CameraView,
}

//TODO: Fix this whole mess

const SENSITIVITY: f32 = 0.02;

pub fn camera_ortho_controller(
    active_split: Res<ActiveSplit>,
    _time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraOrthoController, &mut Projection)>,
) {
    if let ActiveSplit::View(cam, center) = &*active_split {
        for (mut transform, controller, mut projection) in query.iter_mut() {
            if controller.view == *cam {
                if let Orthographic(p) = &mut *projection {
                    let mut zoom = 0.0;

                    for ev in scroll_evr.iter() {
                        match ev.unit {
                            MouseScrollUnit::Line => {
                                // println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                                zoom += ev.y;
                            }
                            MouseScrollUnit::Pixel => {
                                // println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                                zoom += ev.y
                            }
                        }
                    }

                    p.scale -= zoom;
                    p.scale = p.scale.max(1.0);
                }

                if key_input.pressed(KeyCode::Space)
                    && mouse_button_input.pressed(MouseButton::Left)
                {
                    let mut drag = Vec2::ZERO;
                    for ev in mouse_events.iter() {
                        drag += ev.delta
                    }

                    drag *= SENSITIVITY;

                    let mut pos = transform.translation;
                    match cam {
                        CameraView::View3D => unreachable!(),
                        CameraView::Top => {
                            pos.x += drag.x;
                            pos.z += drag.y;
                        }
                        CameraView::Side => {
                            pos.x -= drag.x; // really not sure why the minus is needed here...
                            pos.y += drag.y;
                        }
                        CameraView::Front => {
                            pos.z += drag.x;
                            pos.y += drag.y;
                        }
                    }
                    *transform = transform.with_translation(pos);

                    for mut window in &mut windows {
                        if !window.focused {
                            continue;
                        }

                        window.cursor.grab_mode = CursorGrabMode::Locked;
                        window.cursor.visible = false;

                        // CursorGrabMode::Locked doesn't seem to do anything, so I use this little hack. Have to flip y for some reason
                        let height = window.physical_height();
                        window.set_physical_cursor_position(Some(DVec2::new(
                            center.x as _,
                            height.saturating_sub(center.y) as _,
                        )));
                    }
                }
            }
        }
    } else {
        // Make sure we aren't grabbing the cursor if we aren't active
        for mut window in &mut windows {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}
