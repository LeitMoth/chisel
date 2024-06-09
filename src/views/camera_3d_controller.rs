use bevy::math::DVec2;
use bevy::window::CursorGrabMode;
use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::*;
use std::fmt;

use super::split::{ActiveSplit, CameraView, View3DCamera};

/*
Taken from the bevy scene-viewer example
 */

pub const RADIANS_PER_DOT: f32 = 1.0 / 300.0;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    // pub keyboard_key_enable_mouse: KeyCode,
    pub key_enable_move: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyE,
            key_down: KeyCode::KeyQ,
            key_run: KeyCode::ShiftLeft,
            mouse_key_enable_mouse: MouseButton::Left,
            // keyboard_key_enable_mouse: KeyCode::M,
            key_enable_move: KeyCode::Space,
            walk_speed: 5.0,
            run_speed: 15.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        /*
                write!(
                    f,
                    "
        Freecam Controls:
            MOUSE\t- Move camera orientation
            {:?}/{:?}\t- Enable mouse movement
            {:?}{:?}\t- forward/backward
            {:?}{:?}\t- strafe left/right
            {:?}\t- 'run'
            {:?}\t- up
            {:?}\t- down",
                    self.mouse_key_enable_mouse,
                    self.keyboard_key_enable_mouse,
                    self.key_forward,
                    self.key_back,
                    self.key_left,
                    self.key_right,
                    self.key_run,
                    self.key_up,
                    self.key_down
                )
         */
    }
}

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller);
    }
}

fn camera_controller(
    active_split: Res<ActiveSplit>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    _move_toggled: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<View3DCamera>>,
) {
    let dt = time.delta_seconds();

    if let (Ok((mut transform, mut options)), ActiveSplit::View(CameraView::View3D, center)) =
        (query.get_single_mut(), &*active_split)
    {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }
        // if key_input.just_pressed(options.keyboard_key_enable_mouse) {
        //     *move_toggled = !*move_toggled;
        // }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += right * options.velocity.x * dt
            + Direction3d::Y * options.velocity.y * dt
            + forward * options.velocity.z * dt;

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        //TODO match the spacebar behavoir of hammer better, maybe look at what the original move_toggle was doing
        if key_input.pressed(options.key_enable_move)
            && mouse_button_input.pressed(options.mouse_key_enable_mouse)
        {
            for mouse_event in mouse_events.read() {
                mouse_delta += mouse_event.delta;
            }

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
        if mouse_button_input.just_released(options.mouse_key_enable_mouse)
        /* || key_input.just_released(options.key_enable_move) */
        {
            for mut window in &mut windows {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            options.pitch = (options.pitch - mouse_delta.y * RADIANS_PER_DOT * options.sensitivity)
                .clamp(-PI / 2., PI / 2.);
            options.yaw -= mouse_delta.x * RADIANS_PER_DOT * options.sensitivity;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
        }
    } else {
        // Make sure we aren't grabbing the cursor if we aren't active
        for mut window in &mut windows {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}
