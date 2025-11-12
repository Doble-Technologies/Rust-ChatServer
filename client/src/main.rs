use eframe::egui;
use std::sync::Arc;

mod components;

fn main() -> eframe::Result<()> {
    let icon_data = eframe::icon_data::from_png_bytes(
            include_bytes!("../resources/dti.png")
        )
        .expect("Failed to load icon");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            icon: Some(Arc::new(icon_data)),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Doble Technologies - Chat Application",
        options,
        Box::new(|_cc| Ok(Box::new(ChatApp {})))
    )
}

struct ChatApp;

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut send_string = String::from("Send");

        let window = ctx.content_rect();
        let margin = 40.0;
        egui::Area::new(egui::Id::new("bottom_send_area"))
            .fixed_pos(egui::pos2(
                window.max.x - (margin * 1.5),
                window.max.y - margin
            )).show(ctx, |ui| {
            if let Some(msg) = components::string_button::string_button(ui, &mut send_string, "Send") {
                println!("Send Clicked! {}", msg);
                send_string.clear();
            }
        });
    }
}