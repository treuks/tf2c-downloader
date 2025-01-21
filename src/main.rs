#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod tf2theme;

use core::fmt;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
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

enum ErrLabel {
    NoSteamApps,
    NoSourcemods,
    PopulateErr(PopulateError),
}

impl fmt::Display for ErrLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrLabel::NoSteamApps => write!(f, "No steamapps in selected location"),
            ErrLabel::NoSourcemods => write!(f, "No sourcemods in steamapps"),

            ErrLabel::PopulateErr(PopulateError::VersionFileParseError) => write!(
                f,
                "There's a version file, but we couldn't parse it. Is the format changed?"
            ),
            ErrLabel::PopulateErr(PopulateError::SourcemodsEmpty) => write!(f, "This isn't really an error, the game is just not installed."),
            ErrLabel::PopulateErr(PopulateError::NoVersionFile) => write!(f, "There's no version file in the game location."),
            ErrLabel::PopulateErr(PopulateError::NoSourcemods) => write!(f, "There's no \"sourcemods\" folder in that steam library. Is it your main steam library?")
        }
    }
}

struct MyApp {
    steam_folder_location: Option<PathBuf>,
    location_err: Option<ErrLabel>,
    game_data: Option<GameData>,
}

fn parse_version_file(version_file: &str) -> HashMap<String, String> {
    let mut kv: HashMap<String, String> = HashMap::new();
    for line in version_file.lines() {
        let split: Vec<&str> = line.split("=").collect();
        kv.insert(split[0].to_string(), split[1].to_string());
    }
    kv
}

fn sus_to_version(map: &HashMap<String, String>) -> Option<Version> {
    let name = map.get("VersionName")?;
    let time = map.get("VersionTime")?;

    Some(Version {
        version_name: name.to_owned(),
        version_time: time.to_owned(),
    })
}

struct Version {
    version_time: String,
    version_name: String,
}

fn get_game_data(steam_folder_location: &PathBuf) -> Result<GameData, PopulateError> {
    let mut location = steam_folder_location.clone();
    location.push("steamapps/sourcemods/tf2classic");

    match location.try_exists() {
        Ok(true) => {
            let game_version = std::fs::read_to_string(Path::join(&location, "version.txt"));
            match game_version {
                Err(_) => return Err(PopulateError::NoVersionFile),
                Ok(version) => {
                    let version_file = parse_version_file(&version);
                    let Some(version) = sus_to_version(&version_file) else {
                        return Err(PopulateError::VersionFileParseError);
                    };

                    Ok(GameData { location, version })
                }
            }
        }
        Ok(false) => {
            location.pop();
            let Ok(true) = location.try_exists() else {
                return Err(PopulateError::NoSourcemods);
            };
            Err(PopulateError::SourcemodsEmpty)
        }
        _ => unimplemented!(),
    }
}

enum PopulateError {
    SourcemodsEmpty,
    NoSourcemods,
    NoVersionFile,
    VersionFileParseError,
}

struct GameData {
    location: PathBuf,
    version: Version,
}

impl Default for MyApp {
    fn default() -> Self {
        // Self {
        //     steam_folder_location: get_default_steam_location(),
        //     game_data: None,
        //     location_err: None,
        // }
        let default_steam_location = get_default_steam_location();
        match default_steam_location {
            None => Self {
                steam_folder_location: None,
                game_data: None,
                location_err: None,
            },
            Some(location) => {
                let game_data = get_game_data(&location);
                match game_data {
                    Ok(gd) => Self {
                        steam_folder_location: Some(location),
                        game_data: Some(gd),
                        location_err: None,
                    },
                    Err(er) => Self {
                        steam_folder_location: Some(location),
                        game_data: None,
                        location_err: Some(ErrLabel::PopulateErr(er)),
                    },
                }
            }
        }
    }
}

// fn big_button(text: &str) -> Button {
//     Button::new(RichText::new(text).size(14.0))
// }

fn get_default_steam_location() -> Option<PathBuf> {
    // TODO: use the windows registry to find a path instead/alongside of this
    let default_location = Path::new(r#"C:\Program Files (x86)\Steam"#);

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
                                let game_data = get_game_data(&location);
                                match game_data {
                                    Ok(gd) => app.game_data = Some(gd),
                                    Err(er) => {
                                        app.location_err = Some(ErrLabel::PopulateErr(er));
                                        app.steam_folder_location = Some(location);
                                        return;
                                    }
                                }
                            } else {
                                app.steam_folder_location = None;
                                app.location_err = Some(ErrLabel::NoSteamApps);
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
                        match &self.game_data {
                            Some(gd) => {
                                ui.add_space(8.0);
                                ui.label(format!(
                                    "Installed TF2C version: {}",
                                    gd.version.version_name
                                ));
                            }
                            None => match &self.location_err {
                                None => {
                                    ui.label("XDD there's no game data and no error so something is really wrong");
                                }
                                Some(err) => {
                                    ui.label(format!("Error: {}", err.to_string()));
                                }
                            },
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
