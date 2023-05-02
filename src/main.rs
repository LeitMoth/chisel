use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use init::{setup_system, wiggle_system};
use ui::ChiselUIPlugin;
use views::split::ChiselCamerasPlugin;

mod init;
mod ui;
mod views;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ChiselUIPlugin)
        .add_plugin(ChiselCamerasPlugin)
        .add_startup_system(setup_system)
        .add_system(wiggle_system)
        .run();
}
