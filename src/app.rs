use crate::gcode_loader::GCodeLoader;
use egui::{FontId, RichText};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct TemplateApp {
    jog_speed: f32,
    serial_port: String,
    gcode_loader: GCodeLoader,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}
impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            serial_port: "".to_owned(),
            jog_speed: 0.001,
            gcode_loader: GCodeLoader::default(),
            file_dialog: FileDialog::new(),
            picked_file: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Default::default()
    }

    fn custom_text(text: &str, size: f32) -> RichText {
        RichText::new(text).font(FontId::proportional(size))
    }
}

const SUBHEADER_SIZE: f32 = 15.0;

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            serial_port,
            jog_speed,
            gcode_loader,
            file_dialog,
            picked_file,
        } = self;
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Plotting settings");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Select serial port");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{serial_port}"))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(serial_port, "/dev/tty1".parse().unwrap(), "tty1");
                        });
                    ui.add_space(16.0);
                    ui.label(format!("Current file: {:?}", self.picked_file));
                    if let Some(path) = self.file_dialog.update(ctx).picked() {
                        self.picked_file = Some(path.to_path_buf());
                        self.gcode_loader =
                            GCodeLoader::new(self.picked_file.clone().expect("REASON"));
                    }
                    if ui.button("Select file").clicked() {
                        self.file_dialog.pick_file();
                    };
                    ui.add_space(16.0);
                    ui.label("Set speed");
                    ui.add(
                        egui::Slider::new(jog_speed, 0.001..=0.100).text("Speed per jog (inches)"),
                    );
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        let _ = ui.button("Start plotting");
                        let _ = ui.button("Abort plotting");
                    });
                });
                ui.vertical(|ui| {
                    ui.label(Self::custom_text("GCode file", SUBHEADER_SIZE));
                    ui.label(format!("Lines: {:?}", self.gcode_loader.gcode.len()));
                    ui.add_space(10.0);
                    ui.label(Self::custom_text("GRBL settings", SUBHEADER_SIZE));
                    ui.label("-");
                });
            });

            ui.separator();
            ui.heading("Manual Control");
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let _ = ui.button("Zero machine");
                    let _ = ui.button("Go home");
                });
                ui.vertical(|ui| {
                    ui.label("Jog the head in X and Y axis");
                    egui::Grid::new("parent grid").striped(true).show(ui, |ui| {
                        ui.label("");
                        let _ = ui.button("⬆");
                        ui.label("");

                        ui.end_row();

                        let _ = ui.button("⬅");
                        ui.label("");
                        let _ = ui.button("➡");

                        ui.end_row();

                        ui.label("");
                        let _ = ui.button("⬇");
                        ui.label("");

                        ui.end_row();
                    });
                });
            });
            ui.add_space(10.0);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
