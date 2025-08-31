use crate::gcode_loader::GCodeLoader;
use egui::{FontId, RichText, ScrollArea, TextStyle};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct TemplateApp {
    jog_speed: f32,
    serial_port: String,
    gcode_loader: GCodeLoader,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    serial_monitor: Vec<String>,
}
impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            serial_port: "".to_owned(),
            jog_speed: 0.001,
            gcode_loader: GCodeLoader::default(),
            file_dialog: FileDialog::new(),
            picked_file: None,
            serial_monitor: vec![],
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
            serial_monitor,
        } = self;
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::SidePanel::right("right_panel")
            .show(ctx, |ui| {
                ui.heading("Manual Control");
                ui.add_space(16.0);
                let _ = ui.button("Zero machine").on_hover_text("Set home to the current location");
                if ui.button("Go home").on_hover_text("Move the head to (0, 0)").clicked(){
                    self.serial_monitor.push("homehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehomehome".to_string());
                };
                let _ = ui.button("Lift the head").on_hover_text("De facto Z axis jog");
                ui.add_space(10.0);
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
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Plotting settings");
            ui.add_space(10.0);
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
                self.gcode_loader = GCodeLoader::new(self.picked_file.clone().expect("REASON"));
            }
            if ui.button("Select file").clicked() {
                self.file_dialog.pick_file();
            };
            ui.add_space(16.0);
            ui.label("Set speed");
            ui.add(egui::Slider::new(jog_speed, 0.001..=0.100).text("Speed per jog (inches)"));
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                let _ = ui.button("Start plotting").on_hover_text("Begin streaming GCode");
                let _ = ui.button("Abort plotting").on_hover_text("Stop streaming GCode (this is NOT immediate)");
            });

            ui.separator();
            ui.heading("Status");
            ui.add_space(10.0);

            ui.label(Self::custom_text("GCode file", SUBHEADER_SIZE));
            ui.label(format!("Lines: {:?}", self.gcode_loader.gcode.len()));
            ui.add_space(10.0);
            ui.label(Self::custom_text("GRBL settings", SUBHEADER_SIZE));
            ui.label("-");
            ui.label(Self::custom_text("Serial monitor", SUBHEADER_SIZE));

            ui.add_space(4.0);

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().stick_to_bottom(true).show_rows(
                ui,
                row_height,
                self.serial_monitor.len(),
                |ui, _row_range| {
                    for row in &self.serial_monitor {
                        let text = format!("{}", row);
                        ui.label(text);
                    }
                },
            );
            ui.ctx().request_repaint();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
