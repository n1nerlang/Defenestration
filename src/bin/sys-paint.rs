use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "defenestration_paint",
        options,
        Box::new(|_cc| Ok(Box::new(PaintApp::new()))),
    )
}

/// Represents a single continuous brush stroke
struct StrokePath {
    points: Vec<egui::Pos2>,
    color: egui::Color32,
    width: f32,
}

struct PaintApp {
    strokes: Vec<StrokePath>,
    current_stroke: Vec<egui::Pos2>,
    brush_color: egui::Color32,
    brush_width: f32,
}

impl PaintApp {
    fn new() -> Self {
        Self {
            strokes: Vec::new(),
            current_stroke: Vec::new(),
            brush_color: egui::Color32::from_rgb(255, 255, 255), // Start with white ink
            brush_width: 4.0,
        }
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        // 1. Classic Retro Side Control Panel
        egui::SidePanel::left("toolbox")
            .resizable(false)
            .default_width(140.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.heading("🪟 Paint");
                    ui.small("Defenestration Edition");
                    ui.add_space(10.0);
                    ui.separator();
                });

                ui.add_space(15.0);
                ui.label(egui::RichText::new("Brush Size").strong());
                ui.add(egui::Slider::new(&mut self.brush_width, 1.0..=32.0).text("px"));

                ui.add_space(20.0);
                ui.label(egui::RichText::new("Palette").strong());
                
                // Grid layout for color selection boxes
                egui::Grid::new("color_grid").spacing([8.0, 8.0]).show(ui, |ui| {
                    let colors = [
                        ("🔴", egui::Color32::from_rgb(242, 80, 34)),   // Windows Red
                        ("🟢", egui::Color32::from_rgb(127, 186, 0)),   // Windows Green
                        ("🔵", egui::Color32::from_rgb(0, 164, 239)),   // Windows Blue
                        ("🟡", egui::Color32::from_rgb(255, 185, 0)),   // Windows Yellow
                        ("⚪", egui::Color32::WHITE),
                        ("⚫", egui::Color32::BLACK),
                        ("🟣", egui::Color32::from_rgb(128, 0, 128)),
                        ("🟠", egui::Color32::from_rgb(255, 165, 0)),
                    ];

                    for (idx, &(icon, color)) in colors.iter().enumerate() {
                        let btn = egui::Button::new(icon).fill(if self.brush_color == color {
                            egui::Color32::from_rgb(60, 60, 60)
                        } else {
                            egui::Color32::TRANSPARENT
                        });
                        
                        if ui.add(btn).clicked() {
                            self.brush_color = color;
                        }
                        if (idx + 1) % 2 == 0 {
                            ui.end_row();
                        }
                    }
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(10.0);

                // Action Utilities
                if ui.button("🗑️ Clear Canvas").clicked() {
                    self.strokes.clear();
                    self.current_stroke.clear();
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.weak(format!("Strokes: {}", self.strokes.len()));
                });
            });

        // 2. Continuous Drawing Canvas Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Setup drawing canvas area boundary
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
            
            // Draw a subtle framing edge around our interactive canvas bounds
            painter.rect_stroke(response.rect, 2.0, egui::Stroke::new(1.0, egui::Color32::from_gray(50)));

            // Capture incoming mouse clicks/drags directly over the drawing region
            if response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    // Only log pointer coordinates if they fall inside the legal box boundary
                    if response.rect.contains(pointer_pos) {
                        // Prevent tracking duplicate coordinates to optimize vector memory space
                        if self.current_stroke.last() != Some(&pointer_pos) {
                            self.current_stroke.push(pointer_pos);
                        }
                    }
                }
            } else if response.drag_released() && !self.current_stroke.is_empty() {
                // Mouse released; freeze path vector array allocation to active historical record list
                self.strokes.push(StrokePath {
                    points: self.current_stroke.clone(),
                    color: self.brush_color,
                    width: self.brush_width,
                });
                self.current_stroke.clear();
            }

            // --- Rendering Engine Pipeline Pass ---
            
            // Draw all completed historical path lines stored in memory
            for stroke in &self.strokes {
                if stroke.points.len() > 1 {
                    for window in stroke.points.windows(2) {
                        painter.line_segment([window[0], window[1]], egui::Stroke::new(stroke.width, stroke.color));
                    }
                } else if let Some(&lone_point) = stroke.points.first() {
                    painter.circle_filled(lone_point, stroke.width / 2.0, stroke.color);
                }
            }

            // Draw the current line currently active under the pointer tip
            if self.current_stroke.len() > 1 {
                for window in self.current_stroke.windows(2) {
                    painter.line_segment([window[0], window[1]], egui::Stroke::new(self.brush_width, self.brush_color));
                }
            } else if let Some(&lone_point) = self.current_stroke.first() {
                painter.circle_filled(lone_point, self.brush_width / 2.0, self.brush_color);
            }
        });
    }
}
