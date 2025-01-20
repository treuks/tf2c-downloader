#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod tf2theme;

use core::fmt;
use std::{
    path::{Path, PathBuf},
};


use egui::{style::Selection, Button, Color32, Id, RichText, Rounding, Stroke, Style, Theme, Ui};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TF2C Updater",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            tf2theme::setup_tf2theme(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

enum LocationError {
    NoSteamApps,
    NoSourcemods,
}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LocationError::NoSteamApps => write!(f, "No steamapps in selected location"),
            LocationError::NoSourcemods => write!(f, "No sourcemods in steamapps"),
        }
    }
}

struct MyApp {
    steam_folder_location: Option<PathBuf>,
    location_err: Option<LocationError>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            steam_folder_location: get_default_steam_location(),
            location_err: None,
        }
    }
}

// fn big_button(text: &str) -> Button {
//     Button::new(RichText::new(text).size(14.0))
// }

fn get_default_steam_location() -> Option<PathBuf> {
    // TODO: use the windows registry to find a path instead/alongside of this
    let default_location = Path::new(r#"C:\Program Files (x86)\Steam\"#);

    let location_path_buf = default_location.to_path_buf();

    if default_location.is_dir() && location_has_dir(&location_path_buf, "steamapps") {
        Some(location_path_buf)
    } else {
        None
    }
}

fn location_has_dir(location: &Path, dirname: &str) -> bool {
    match location.read_dir() {
        Ok(dir) => dir.filter_map(|entry| entry.ok()).any(|entry| {
            entry.file_name() == dirname
                && match entry.metadata() {
                    Err(_) => false,
                    Ok(xd) => xd.is_dir(),
                }
        }),
        Err(_) => false,
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = (8.0, 8.0).into();

            ui.vertical(|ui| {
                let pick_steam_location = |app: &mut MyApp| {
                    let steam_location = rfd::FileDialog::new().pick_folder();

                    match steam_location {
                        None => {}
                        Some(location) => {
                            if location_has_dir(&location, "steamapps") {
                                app.steam_folder_location = Some(location);
                                app.location_err = None;
                            } else {
                                app.steam_folder_location = None;
                                app.location_err = Some(LocationError::NoSteamApps);
                            }
                        }
                    }
                };

                match &self.steam_folder_location {
                    None => {
                        match &self.location_err {
                            None => {
                                ui.label("No steam location selected");
                            }
                            Some(er) => {
                                ui.label(er.to_string());
                            }
                        }

                        if ui
                            .button(format!(
                                "{} Select a location..",
                                egui_phosphor::regular::FILES
                            ))
                            .clicked()
                        {
                            pick_steam_location(self);
                        }
                    }
                    Some(location) => {
                        ui.label(format!(
                            "Location selected: {}",
                            &location.to_string_lossy()
                        ));
                        if ui
                            .button(format!(
                                "{}  Select a different location..",
                                egui_phosphor::regular::ARROW_ARC_LEFT
                            ))
                            .clicked()
                        {
                            pick_steam_location(self);
                        }
                    }
                }
            });
            // ui.allocate_new_ui(
            //     egui::UiBuilder::new().layout(egui::Layout::right_to_left(egui::Align::RIGHT)),
            //     |ui| {
            //         ui.add(big_button(&format!(
            //             "{} destroy orphans",
            //             egui_phosphor::regular::KNIFE
            //         )));

            //         ui.add(big_button(&format!(
            //             "{} destroy orphans",
            //             egui_phosphor::regular::KNIFE
            //         )));
            //     },
            // )
        });
    }
}
