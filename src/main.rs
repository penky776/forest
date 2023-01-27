use eframe::egui;
use egui::Ui;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

fn main() {
    // application
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 650.0)),
        ..Default::default()
    };

    eframe::run_native(
        "forest",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.separator();
                ui.heading("entry3");
                let entry3_main = list_all_files("entry3".to_string());
                browse_dir(entry3_main, ui, &"entry3/".to_string())
            });

        egui::SidePanel::left("left_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.separator();
                ui.heading("entry1");
                let entry1_main = list_all_files("entry1".to_string());
                browse_dir(entry1_main, ui, &"entry1/".to_string())
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("entry2");
            let entry2_main = list_all_files("entry2".to_string());
            browse_dir(entry2_main, ui, &"entry2/".to_string())
        });
    }
}

fn browse_dir(entry: Result<Vec<PathBuf>, MyError>, ui: &mut Ui, main_dir: &String) {
    for file in entry.unwrap().iter() {
        if check_is_file(file.to_path_buf()) {
            make_button(file, ui);
        } else {
            let subdir = main_dir.to_owned() + "/" + &get_file_name(file.to_path_buf()) + "/";
            let test = ui.collapsing(get_file_name(file.to_path_buf()), |_ui| {});
            if test.fully_open() {
                browse_dir(list_all_files(subdir.to_owned()), ui, &subdir)
            }
        }
    }
}

fn make_button(file: &PathBuf, ui: &mut Ui) {
    if ui.button(get_file_name(file.to_path_buf())).clicked() {
        Command::new("xdg-open")
            .arg(file)
            .spawn()
            .expect("command failed");
    }
}

fn list_all_files(entry: String) -> Result<Vec<PathBuf>, MyError> {
    let mut files = Vec::with_capacity(50); // limit of 50 files/folders in each directory
    let entries = fs::read_dir("/home/potato/Code/forest/dir/".to_owned() + &entry);
    match entries {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    files.push(entry.path());
                };
            }
            return Ok(files);
        }
        Err(_) => Err(MyError::new("something went wrong...")),
    }
}

fn get_file_name(path: PathBuf) -> String {
    return path.file_name().unwrap().to_string_lossy().to_string();
}

fn check_is_file(path: PathBuf) -> bool {
    return path.is_file();
}
