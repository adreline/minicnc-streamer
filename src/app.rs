use crate::cnc_machine::CNCMachine;
use crate::cncmachine::list_serial_ports;
use crate::gcode_loader::GCodeLoader;
use egui::{Color32, FontId, RichText, ScrollArea, TextStyle};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct TemplateApp {
    jog_speed: f32,
    serial_port: String,
    gcode_loader: GCodeLoader,
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
    serial_monitor: Vec<String>,
    machine: CNCMachine,
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
            machine: CNCMachine::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Default::default()
    }

    fn subheader(text: &str) -> RichText {
        RichText::new(text)
            .font(FontId::proportional(15.0))
            .color(Color32::from_rgb(255, 238, 219))
    }
    fn header(text: &str) -> RichText {
        RichText::new(text)
            .font(FontId::proportional(25.0))
            .color(Color32::from_rgb(209, 122, 34))
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                ui.label(Self::header("Manual Control"));
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
            ui.label(Self::header("Plotting settings"));
            ui.add_space(10.0);
            ui.label("Select serial port");

            let mut serial_port_copy = self.serial_port.clone();
            egui::ComboBox::from_label("")
                .selected_text(format!("{serial_port_copy}"))
                .show_ui(ui, |ui| {
                    for serial_port_info in list_serial_ports() {
                        ui.selectable_value(
                            &mut serial_port_copy,
                            serial_port_info.port_name.clone(),
                            serial_port_info.port_name.clone(),
                        );
                    }
                });
            if self.serial_port != serial_port_copy {
                self.serial_port = serial_port_copy;
                self.serial_monitor.push("Serial port set".to_string());
            }

            ui.add_space(16.0);
            ui.label(format!("Current file: {:?}", self.picked_file));
            if let Some(path) = self.file_dialog.update(ctx).picked() {
                self.picked_file = Some(path.to_path_buf());
                self.gcode_loader = GCodeLoader::new(self.picked_file.clone().expect("REASON"));
            }
            ui.horizontal(|ui| {
                if ui.button("Select file").clicked() {
                    self.file_dialog.pick_file();
                };
                if ui
                    .button("Inspect file")
                    .on_hover_text("Dumps the pre-processed GCode file to serial monitor")
                    .clicked()
                {
                    for row in self.gcode_loader.gcode.clone() {
                        self.serial_monitor.push(row);
                    }
                };
            });

            ui.add_space(16.0);
            ui.label("Set speed");
            ui.add(
                egui::Slider::new(&mut self.jog_speed, 0.001..=0.100)
                    .text("Speed per jog (inches)"),
            );
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                let _ = ui
                    .button("Start plotting")
                    .on_hover_text("Begin streaming GCode");
                let _ = ui
                    .button("Abort plotting")
                    .on_hover_text("Stop streaming GCode (this is NOT immediate)");
            });

            ui.separator();
            ui.label(Self::header("Status"));
            ui.add_space(10.0);

            ui.label(Self::subheader("GCode file"));
            ui.label(format!("Lines: {:?}", self.gcode_loader.gcode.len()));
            ui.add_space(10.0);
            ui.label(Self::subheader("GRBL settings"));
            ui.label("-");
            ui.label(Self::subheader("Serial monitor"));

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
