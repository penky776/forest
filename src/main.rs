mod error;

use eframe::egui;
use egui::Ui;
use error::ForestError;
use std::{fs, io, path::PathBuf, process::Command};

fn main() {
    // application
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 650.0)),
        ..Default::default()
    };

    let mut dir = String::new();

    println!("enter parent directory: ");
    io::stdin()
        .read_line(&mut dir)
        .expect("couldn't read input");

    let dir = match dir.trim().parse() {
        Ok(dir) => Ok(dir),
        Err(_) => Err(ForestError::UnableToParse),
    };

    eframe::run_native(
        "forest",
        native_options,
        Box::new(|_cc| Box::new(MyApp { dir: dir.unwrap() })),
    );
}

struct MyApp {
    dir: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let parent_dir = list_three_folders(&self.dir).unwrap();

        egui::SidePanel::right("right_panel")
            .default_width(300.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.heading(get_file_name(&parent_dir[0]));
                    let entry3_main = list_all_files(get_file_name(&parent_dir[0]), &self.dir);
                    browse_dir(entry3_main, ui, &get_file_name(&parent_dir[0]), &self.dir)
                });
            });

        egui::SidePanel::left("left_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.heading(get_file_name(&parent_dir[1]));
                    let entry1_main = list_all_files(get_file_name(&parent_dir[1]), &self.dir);
                    browse_dir(entry1_main, ui, &get_file_name(&parent_dir[1]), &self.dir)
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.separator();
                ui.heading(get_file_name(&parent_dir[2]));
                let entry2_main = list_all_files(get_file_name(&parent_dir[2]), &self.dir);
                browse_dir(entry2_main, ui, &get_file_name(&parent_dir[2]), &self.dir)
            });
        });
    }
}

fn browse_dir(
    entry: Result<Vec<PathBuf>, ForestError>,
    ui: &mut Ui,
    main_dir: &String,
    parent_dir: &String,
) {
    for (i, file) in entry.unwrap().iter().enumerate() {
        if check_is_file(file.to_path_buf()) {
            make_button(file, ui);
        } else {
            let subdir = main_dir.to_owned() + "/" + &get_file_name(&file.to_path_buf()) + "/";

            let id = ui.make_persistent_id(i);

            ui.push_id(id, |ui| {
                let collapser = ui.collapsing(get_file_name(&file.to_path_buf()), |_ui| {});
                if collapser.fully_open() {
                    browse_dir(
                        list_all_files(subdir.to_owned(), parent_dir),
                        ui,
                        &subdir,
                        parent_dir,
                    )
                }
            });
        }
    }
}

fn make_button(file: &PathBuf, ui: &mut Ui) {
    if ui.button(get_file_name(&file.to_path_buf())).clicked() {
        Command::new("xdg-open")
            .arg(file)
            .spawn()
            .expect("command failed");
    }
}

// get three folders from parent directory
fn list_three_folders(parent_dir: &String) -> Result<Vec<PathBuf>, ForestError> {
    let mut folders = Vec::with_capacity(3); // limit of THREE folders.
    let entries = fs::read_dir(parent_dir);

    match entries {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if check_is_file(entry.path()) == false && folders.len() < 3 {
                        folders.push(entry.path());
                    } else if check_is_file(entry.path()) == true && folders.len() < 3 {
                        continue;
                    } else {
                        return Ok(folders);
                    }
                }
            }
            return Ok(folders);
        }
        Err(_) => Err(ForestError::FailedToReadDir),
    }
}

fn list_all_files(entry: String, parent_dir: &String) -> Result<Vec<PathBuf>, ForestError> {
    let mut files = Vec::with_capacity(200); // limit of 200 files/folders in each directory
    let entries = fs::read_dir(parent_dir.to_owned() + &entry);

    match entries {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    files.push(entry.path());
                };
            }
            return Ok(files);
        }
        Err(_) => Err(ForestError::FailedToReadDir),
    }
}

fn get_file_name(path: &PathBuf) -> String {
    return path.file_name().unwrap().to_string_lossy().to_string();
}

fn check_is_file(path: PathBuf) -> bool {
    return path.is_file();
}
