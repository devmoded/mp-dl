use eframe::egui;
use log::{info, warn};
// use std::path::PathBuf;

use crate::backend::path::get_minecraft_dir;

pub fn run_gui() -> eframe::Result {
    egui_logger::builder()
        .add_blacklist("wgpu_hal::gles::egl")
        .add_blacklist("wgpu_hal::vulkan::adapter")
        .add_blacklist("egui_wgpu")
        .add_blacklist("zbus::connection::handshake::common")
        .add_blacklist("zbus::proxy")
        .init().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("mp-dl запущен");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "mp-dl GUI",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

pub struct MyApp {
    url: String,
    unpack_path: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let default_path = directories::UserDirs::new()
            .and_then(|dirs| dirs.download_dir().map(|p| p.to_string_lossy().into_owned()))
            .unwrap_or_else(|| ".".to_string());
        Self {
            url: String::new(),
            unpack_path: default_path,
        }
    }
}

// impl MyApp {
//     pub fn get_unpack_path(&self) -> PathBuf {
//         PathBuf::from(&self.unpack_path)
//     }
// }

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("mp-dl GUI");
            ui.add_space(8.0);

            ui.label("URL:");
            ui.add(egui::TextEdit::singleline(&mut self.url).hint_text("Введите URL архива сборки...").desired_width(f32::INFINITY));

            ui.add_space(4.0);

            ui.label("Путь распаковки:");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.button("Обзор...").clicked() {
                    let start_dir = get_minecraft_dir().expect("Не удалось получить путь до .minecraft");

                    let dialog = rfd::FileDialog::new()
                        .set_title("Выберите папку для распаковки")
                        .set_directory(&start_dir);
                    if let Some(unpack_dir) = dialog.pick_folder() {
                        self.unpack_path = unpack_dir.to_string_lossy().into_owned();
                        info!("Выбран путь распаковки {}", self.unpack_path);
                    }
                }

                ui.add_space(10.0);

                ui.add(egui::TextEdit::singleline(&mut self.unpack_path).desired_width(f32::INFINITY));
            });

            ui.add_space(12.0);

            if ui.add_sized([ui.available_width(), 30.0], egui::Button::new("Загрузить")).clicked() {
                if self.url.trim().is_empty() {
                    warn!("URL не может быть пустым!");
                } else {
                    info!("Начало загрузки {}...", self.url);
                }
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            egui::CollapsingHeader::new("Логи программы")
                .default_open(true)
                .show(ui, |ui| {
                    ui.scope(|ui| {
                        egui_logger::logger_ui().show(ui);
                    });
                });
        });
    }
}
