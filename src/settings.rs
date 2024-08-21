use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub dji_endpoint: String,
    pub dji_api_key: String,
    pub mapbox_token: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dji_endpoint: String::new(),
            dji_api_key: String::new(),
            mapbox_token: String::new(),
        }
    }
}

impl Settings {
    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Settings")
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("DJI API")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("Endpoint:");
                        ui.text_edit_singleline(&mut self.dji_endpoint);
                        ui.label("API Key:");
                        ui.text_edit_singleline(&mut self.dji_api_key);
                    });

                egui::CollapsingHeader::new("Mapbox")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label("Token:");
                        ui.text_edit_singleline(&mut self.mapbox_token);
                    });
            });
    }
}
