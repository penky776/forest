use eframe::egui;
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

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .exact_width(300.0)
            .show(ctx, |ui| {
                ui.heading("entry3");
                let entry3_files = list_all_files("entry3".to_string());
                for file in entry3_files.unwrap().iter() {
                    if ui.button("click me!").clicked() {
                        Command::new("xdg-open")
                            .arg(file)
                            .spawn()
                            .expect("command failed");
                    }
                }
            });

        egui::SidePanel::left("left_panel")
            .exact_width(250.0)
            .show(ctx, |ui| {
                ui.heading("entry1");
                let entry1_files = list_all_files("entry1".to_string());
                for file in entry1_files.unwrap().iter() {
                    if ui.button("click me!").clicked() {
                        Command::new("xdg-open")
                            .arg(file)
                            .spawn()
                            .expect("command failed");
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("entry2");
            let entry2_files = list_all_files("entry2".to_string());
            for file in entry2_files.unwrap().iter() {
                if ui.button("click me!").clicked() {
                    Command::new("xdg-open")
                        .arg(file)
                        .spawn()
                        .expect("command failed");
                }
            }
        });
    }
}

fn list_all_files(entry: String) -> Result<Vec<PathBuf>, MyError> {
    let mut files = Vec::with_capacity(10);
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
