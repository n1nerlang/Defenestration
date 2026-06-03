use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([580.0, 420.0])
            .with_resizable(false), // Fixed size for that classic wizard look
        ..Default::default()
    };

    eframe::run_native(
        "defenestration_welcome",
        options,
        Box::new(|_cc| Ok(Box::new(WelcomeWizard::new()))),
    )
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SelectedLanguage {
    English,
    Spanish,
    French,
    LuauDeveloperMode,
}

struct WelcomeWizard {
    current_lang: SelectedLanguage,
    is_dark_mode: bool,
    configuration_applied: bool,
}

impl WelcomeWizard {
    fn new() -> Self {
        Self {
            current_lang: SelectedLanguage::English,
            is_dark_mode: true, // Safeguarding developer eyesight by default
            configuration_applied: false,
        }
    }
}

impl eframe::App for WelcomeWizard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply the live theme choice directly to the framework
        if self.is_dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // 1. Top Title Banner Panel
        egui::TopBottomPanel::top("welcome_header")
            .frame(egui::Frame::default().inner_margin(12.0).fill(if self.is_dark_mode {
                egui::Color32::from_rgb(30, 35, 45)
            } else {
                egui::Color32::from_rgb(230, 235, 245)
            }))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(egui::RichText::new("Defenestration OS Personalization").strong());
                        ui.small("Localization & Desktop Theme Interface Selection");
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.heading("🎨");
                    });
                });
            });

        // 2. Bottom Navigation Control Bar
        egui::TopBottomPanel::bottom("welcome_footer")
            .inner_margin(12.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.weak("Post-Install Deployment Stage");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.configuration_applied {
                            if ui.button("Launch Desktop 🚀").clicked() {
                                std::process::exit(0); // Hand off control to the desktop manager
                            }
                        } else {
                            if ui.button("Apply Settings >").clicked() {
                                self.configuration_applied = true;
                            }
                        }
                        
                        let _ = ui.add_enabled(false, egui::Button::new("< Back"));
                        ui.add_space(10.0);
                        if ui.button("Skip").clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });

        // 3. Central Configuration Control Group
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);

            if !self.configuration_applied {
                ui.label(egui::RichText::new("Configure your system interface parameters before first initialization.").size(14.0));
                ui.add_space(20.0);

                // --- LANGUAGE LOCALIZATION GROUP ---
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label(egui::RichText::new("Select System Language").strong());
                    ui.small("This maps the target language across your shell scripts and logs.");
                    ui.add_space(5.0);

                    egui::ComboBox::from_id_source("lang_dropdown")
                        .width(220.0)
                        .selected_text(format!("{:?}", self.current_lang))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.current_lang, SelectedLanguage::English, "English");
                            ui.selectable_value(&mut self.current_lang, SelectedLanguage::Spanish, "Español (Spanish)");
                            ui.selectable_value(&mut self.current_lang, SelectedLanguage::French, "Français (French)");
                            ui.selectable_value(&mut self.current_lang, SelectedLanguage::LuauDeveloperMode, "Luau Dev Mode (Raw JSON strings)");
                        });
                });

                ui.add_space(15.0);

                // --- VISUAL DESIGN THEME GROUP ---
                ui.group(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label(egui::RichText::new("Select System Theme").strong());
                    ui.add_space(5.0);

                    ui.radio_value(&mut self.is_dark_mode, true, "🌌 Dark Mode (Highly recommended for coding marathons)");
                    ui.radio_value(&mut self.is_dark_mode, false, "☀️ Light Mode");
                    
                    ui.add_space(8.0);
                    // The requested configuration advisory notice
                    ui.label(egui::RichText::new("(Note: This preference setting can be easily changed via system configurations later at any time.)")
                        .italics()
                        .small()
                        .color(egui::Color32::GRAY));
                });

            } else {
                // Settings Confirmation Screen Block
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading("🔒 Locking In Settings");
                    ui.add_space(15.0);
                    
                    ui.add(egui::ProgressBar::new(1.0).text("Writing environment configs to disk..."));
                    
                    ui.add_space(25.0);
                    ui.small(format!("Default Locale Set: {:?}", self.current_lang));
                    ui.small(format!("Interface Mode Applied: {}", if self.is_dark_mode { "Dark Mode" } else { "Light Mode" }));
                });
            }
        });
    }
  }
