use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use geometry::StandardPlane;
use init::{setup_system, wiggle_system, change};
use ui::ChiselUIPlugin;
use views::split::ChiselCamerasPlugin;
use vmf2::vmf;

mod init;
mod ui;
mod views;
mod vmf2;
mod solidcomp;
mod geometry;

fn main() {
    let s = StandardPlane::new(&vmf::Plane {
        points: [
            vmf::Point {
                x: 1.0,
                y: -2.0,
                z: 0.0
            },
            vmf::Point {
                x: 3.0,
                y: 1.0,
                z: 4.0
            },
            vmf::Point {
                x: 0.0,
                y: -1.0,
                z: 2.0
            },
        ],
    });

    println!("{s:#?}");
    // return;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ChiselUIPlugin)
        .add_plugin(ChiselCamerasPlugin)
        .add_startup_system(setup_system)
        .add_system(wiggle_system)
        .add_system(change)
        .run();
}
