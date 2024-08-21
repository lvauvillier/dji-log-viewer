use std::sync::mpsc::{channel, Receiver, Sender};

use dji_log_parser::DJILog;
use log::info;

use crate::flight_data::FlightData;
use crate::settings::Settings;
use crate::utils::execute;

pub enum LoadFileStatus {
    WaitingForInput,
    Parsing,
    FetchingKeychains,
    Success(Option<FlightData>),
    Error(String),
    ClosedAfterError,
}

#[allow(unused_must_use)]
async fn parse(
    file_name: String,
    bytes: Vec<u8>,
    dji_api_key: &str,
    dji_endpoint: &str,
    ctx: &egui::Context,
    status_sender: Sender<LoadFileStatus>,
) {
    status_sender.send(LoadFileStatus::Parsing);

    let parser = match DJILog::from_bytes(bytes) {
        Ok(parser) => parser,
        Err(e) => {
            status_sender.send(LoadFileStatus::Error(e.to_string()));
            return;
        }
    };

    let keychains = if parser.version >= 13 {
        status_sender.send(LoadFileStatus::FetchingKeychains);
        ctx.request_repaint();

        let keychains_request = match parser.keychains_request() {
            Ok(keychains_request) => keychains_request,
            Err(e) => {
                status_sender.send(LoadFileStatus::Error(e.to_string()));
                return;
            }
        };

        match keychains_request
            .fetch_async(dji_api_key, Some(dji_endpoint))
            .await
        {
            Ok(keychains) => keychains,
            Err(e) => {
                info!("error");
                status_sender.send(LoadFileStatus::Error(e.to_string()));
                return;
            }
        }
    } else {
        Vec::new()
    };

    ctx.request_repaint();

    match parser.frames(Some(keychains)) {
        Ok(frames) => {
            status_sender.send(LoadFileStatus::Success(Some(FlightData::new(
                file_name, frames,
            ))));
        }
        Err(e) => {
            status_sender.send(LoadFileStatus::Error(e.to_string()));
        }
    };

    ctx.request_repaint();
}

pub struct LoadFile {
    status_receiver: Receiver<LoadFileStatus>,
    pub status: LoadFileStatus,
}

impl LoadFile {
    pub fn from_dropped_file(
        dropped_file: &egui::DroppedFile,
        ctx: &egui::Context,
        settings: &Settings,
    ) -> Self {
        let (status_sender, status_receiver) = channel();

        let ctx = ctx.clone();
        let dji_api_key = settings.dji_api_key.clone();
        let dji_endpoint = settings.dji_endpoint.clone();
        if let Some(ref bytes) = dropped_file.bytes {
            let bytes = bytes.to_vec();
            let file_name = dropped_file.name.clone();
            execute(async move {
                parse(
                    file_name,
                    bytes,
                    &dji_api_key,
                    &dji_endpoint,
                    &ctx,
                    status_sender,
                )
                .await
            });
        }

        return Self {
            status_receiver,
            status: LoadFileStatus::Parsing,
        };
    }

    pub fn from_file_picker(ctx: &egui::Context, settings: &Settings) -> Self {
        let (status_sender, status_receiver) = channel();

        let ctx = ctx.clone();
        let dji_api_key = settings.dji_api_key.clone();
        let dji_endpoint = settings.dji_endpoint.clone();

        execute(async move {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .add_filter("Supported files", &["txt"])
                .pick_file()
                .await
            {
                let file_name = file.file_name();
                let bytes = file.read().await;

                parse(
                    file_name,
                    bytes,
                    &dji_api_key,
                    &dji_endpoint,
                    &ctx,
                    status_sender,
                )
                .await
            }
        });

        return Self {
            status_receiver,
            status: LoadFileStatus::WaitingForInput,
        };
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        while let Ok(status) = self.status_receiver.try_recv() {
            self.status = status;
        }

        match self.status {
            LoadFileStatus::Parsing => {
                egui::Window::new("Open File")
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.label("Parse file...");
                    });
            }
            LoadFileStatus::FetchingKeychains => {
                egui::Window::new("Open File")
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Fetching Keychains...");
                        });
                    });
            }
            LoadFileStatus::Error(ref error) => {
                let error = error.clone();
                egui::Window::new("Error")
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .auto_sized()
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.label(error);
                            ui.separator();
                            if ui.button("Close").clicked() {
                                self.status = LoadFileStatus::ClosedAfterError;
                            }
                        });
                    });
            }
            _ => {}
        }
    }
}
