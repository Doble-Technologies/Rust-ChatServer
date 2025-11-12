use eframe::egui;

pub fn string_button(ui: &mut egui::Ui, data: &mut String, label: &str) -> Option<String> {
    let mut clicked = false;

    ui.horizontal(|ui| {
        let button = egui::Button::new(label)
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
            .corner_radius(5.0);

        if ui.add(button).clicked() {
            clicked = true;
        }
    });

    if clicked && !data.trim().is_empty() {
        let message = data.clone();
        data.clear();
        Some(message)
    } else {
        None
    }
}