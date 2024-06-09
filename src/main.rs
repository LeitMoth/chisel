use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use controls::ControlPlugin;
use init::InitPlugin;
use ui::ChiselUIPlugin;
use views::split::ChiselCamerasPlugin;

mod controls;
mod geometry;
mod init;
mod solidcomp;
mod ui;
mod views;
mod vmf2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chisel VMF Viewer".to_string(),
                // apparently this give less latency for the raycaster
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(ChiselUIPlugin)
        .add_plugins(ChiselCamerasPlugin)
        .add_plugins(InitPlugin)
        .add_plugins(ControlPlugin)
        .run();
}
