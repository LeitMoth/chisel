use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Resource)]
pub struct Images {
    select_mode_icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            select_mode_icon: asset_server.load("select_icon.png"),
        }
    }
}

pub struct ChiselUIPlugin;

impl Plugin for ChiselUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<Images>()
        .add_system(ui_system);
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    images: Res<Images>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = contexts.add_image(images.select_mode_icon.clone_weak());
    }

    let ctx = contexts.ctx_mut();

    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Load").clicked() {
                        println!("load")
                    }
                    if ui.button("Save").clicked() {
                        println!("save")
                    }
                });
                egui::menu::menu_button(ui, "Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        println!("undo")
                    }
                    if ui.button("Redo").clicked() {
                        println!("redo")
                    }
                });
            });
        })
        .response
        .rect
        .height();


    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Bottom Text");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .default_width(32.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.add(egui::ImageButton::new(*rendered_texture_id, [32.0,32.0])).clicked() {
                    println!("image click!")
                }
                if ui.add(egui::ImageButton::new(*rendered_texture_id, [32.0,32.0])).clicked() {
                    println!("image click!")
                }
                if ui.add(egui::ImageButton::new(*rendered_texture_id, [32.0,32.0])).clicked() {
                    println!("image click!")
                }
            });
            // ui.label("Left resizeable panel");
            // ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
