// Standard Uses

// Crate Uses
use crate::Gui;

// External Uses
use egui::{Context, ScrollArea, Ui};


fn _render_sub_images_list(app: &Gui, _ctx: &Context, ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
        for image in &app.operator.images {
            ui.label(image.name());
        }
    });
}

