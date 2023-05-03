use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use geometry::StandardPlane;
use init::{change, setup_system};
use ui::ChiselUIPlugin;
use views::split::ChiselCamerasPlugin;
use vmf2::vmf;

mod geometry;
mod init;
mod solidcomp;
mod ui;
mod views;
mod vmf2;

fn main() {
    let s = StandardPlane::new(&vmf::Plane {
        points: [
            vmf::Point {
                x: 1.0,
                y: -2.0,
                z: 0.0,
            },
            vmf::Point {
                x: 3.0,
                y: 1.0,
                z: 4.0,
            },
            vmf::Point {
                x: 0.0,
                y: -1.0,
                z: 2.0,
            },
        ],
    });

    println!("{s:#?}");
    // return;

    App::new()
        // .insert_resource(AmbientLight {
        //     color: Color::WHITE,
        //     brightness: 0.2,
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ChiselUIPlugin)
        .add_plugin(ChiselCamerasPlugin)
        .add_startup_system(setup_system)
        // .add_system(wiggle_system)
        .add_system(change)
        .run();
}
