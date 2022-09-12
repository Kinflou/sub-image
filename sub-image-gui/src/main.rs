// Relative Modules
mod sub;

// Standard Uses

// Crate Uses

use std::path::Path;
// External Uses
use eframe::{egui, Frame};
use egui::{menu, Ui};
use egui::Context;
use sub_image::operator::sub::Operator;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Sub-Image",
                       native_options,
                       Box::new(|cc| Box::new(Gui::new(cc)))
    );
}

#[derive(Default)]
struct Gui {
    operator: Operator,

    allowed_to_close: bool,
    show_confirmation_dialog: bool,
}

impl Gui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::bottom("info_panel").show(ctx, |ui| {
            ui.label("Test");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            show_menu(ctx, ui);
            // show_split(ctx, ui);
            // show_image_editor();
        });

        close_prompt(self, ctx, frame);

        ctx.request_repaint()
    }

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
}

fn show_menu(ctx: &Context, ui: &mut Ui) {
    let path = Option::from(Path::new("/").to_path_buf());
    let mut of = egui_file::FileDialog::open_file(path);
    of.show(ctx);

    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {

            if ui.button("Open").clicked() {
                of.open();
            }
            if ui.button("Open Recent").clicked() {

            }
            if ui.button("Save").clicked() {}
            if ui.button("Save All").clicked() {}
        });

        ui.menu_button("Edit", |ui| {
            if ui.button("Undo").clicked() {}
            if ui.button("Redo").clicked() {}
            if ui.button("Redo").clicked() {}
        });

        ui.menu_button("Subs", |ui| {
            if ui.button("Create").clicked() {}
            if ui.button("Delete").clicked() {}
            ui.separator();
            if ui.button("Save").clicked() {}
        });
    });
}

fn _show_split(ctx: &Context, ui: &mut Ui) {
    egui::SidePanel::left("left_panel").show(ctx, |ui2| {
        ui2.label("Hello World!");
    });
    ui.separator();
    egui::SidePanel::right("right_panel").show(ctx, |ui2| {
        ui2.label("Hello World!");
    });
}

fn _show_image_editor(_ctx: &Context, _ui: &mut Ui) {
    /*
    let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
        // Load the texture only once.
        ui.ctx().load_texture(
            "my-image",
            egui::ColorImage::example(),
            egui::TextureFilter::Linear
        )
    });

    // Show the image:
    ui.image(texture, texture.size_vec2());
    */
}

fn close_prompt(app: &mut Gui, ctx: &Context, frame: &mut Frame) {
    if app.show_confirmation_dialog {
        egui::Window::new("Do you want to quit?")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        app.show_confirmation_dialog = false;
                    }

                    if ui.button("Yes!").clicked() {
                        app.allowed_to_close = true;
                        frame.close();
                    }
                });
            });
    }
}
