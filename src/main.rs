use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSystem};
use init::{change_vmf, setup_system, InitPlugin};
use ui::ChiselUIPlugin;
use views::split::ChiselCamerasPlugin;

mod geometry;
mod init;
mod solidcomp;
mod ui;
mod views;
mod vmf2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ChiselUIPlugin)
        .add_plugin(ChiselCamerasPlugin)
        .add_plugin(InitPlugin)
        .run();
}
