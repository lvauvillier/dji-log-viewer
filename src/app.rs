use crate::flight_data::FlightData;
use crate::load_file::{LoadFile, LoadFileStatus};
use crate::settings::Settings;

pub struct App {
    show_settings: bool,
    settings: Settings,
    flight_data: Option<FlightData>,
    load_file: Option<LoadFile>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut settings = Settings::default();
        if let Some(storage) = cc.storage {
            settings = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self {
            settings,
            show_settings: false,
            flight_data: None,
            load_file: None,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load file
        if let Some(load_file) = self.load_file.as_mut() {
            load_file.show(ctx);

            match &mut load_file.status {
                LoadFileStatus::Success(flight_data) => {
                    self.flight_data = flight_data.take();
                    self.load_file = None;
                }
                LoadFileStatus::ClosedAfterError => self.load_file = None,
                _ => {}
            }
        }

        // Handle dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.load_file = Some(LoadFile::from_dropped_file(
                    &i.raw.dropped_files[0],
                    ctx,
                    &self.settings,
                ));
            }
        });

        // Settings
        self.settings.show(ctx, &mut self.show_settings);

        // Menubar
        egui::TopBottomPanel::top("menubar")
            .min_height(30.0)
            .max_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui
                        .add(egui::Button::new("🗁  Open FlightLog").frame(false))
                        .clicked()
                    {
                        self.load_file = Some(LoadFile::from_file_picker(ui.ctx(), &self.settings));
                        ctx.request_repaint();
                    }

                    ui.separator();

                    if let Some(ref flight_data) = self.flight_data {
                        ui.label(&flight_data.file_name);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.hyperlink_to("", env!("CARGO_PKG_REPOSITORY"));
                        ui.separator();
                        egui::widgets::global_dark_light_mode_switch(ui);
                        ui.separator();

                        if ui
                            .add(egui::Button::new("⛭  Settings").frame(false))
                            .clicked()
                        {
                            self.show_settings = true;
                            //ctx.request_repaint();
                        }
                        ui.separator();
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
    }
}
