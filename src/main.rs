use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use ui::ChiselUIPlugin;
use views::split::{setup_system, update_camera_transform_system};

mod ui;
mod views;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ChiselUIPlugin)
        .add_startup_system(setup_system)
        .add_system(update_camera_transform_system)
        .run();
}
