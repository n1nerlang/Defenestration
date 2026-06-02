use eframe::egui;

// 1. The Unified High-Octane Word Pool
const WORD_POOL: &[&str] = &[
    "Admin", "Reaper", "Kernel", "Void", "Ghost", "Shadow", "Byte", "Matrix", 
    "Root", "Daemon", "Vortex", "Alpha", "Glitch", "Phantom", "Stalker", 
    "Breaker", "Nitro", "Apex", "Cyber", "Vector", "Zero", "Overdrive"
];

// Zero-dependency timestamp-seeded tag generator
fn generate_hacker_tag() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    let pool_len = WORD_POOL.len() as u128;
    let idx1 = (now % pool_len) as usize;
    let mut idx2 = ((now >> 5) % pool_len) as usize;

    if idx1 == idx2 {
        idx2 = (idx2 + 1) % WORD_POOL.len();
    }

    let digits = (now % 9000) + 1000; 
    format!("{}{}{}", WORD_POOL[idx1], WORD_POOL[idx2], digits)
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 450.0])
            .with_resizable(false), // Clean fixed dialog window
        ..Default::default()
    };

    eframe::run_native(
        "defenestration_setup",
        options,
        Box::new(|_cc| Ok(Box::new(SetupWizard::default()))),
    )
}

#[derive(Default)]
enum WizardStep {
    #[default]
    Welcome,
    UserConfig,
    TargetDisk,
    Installing,
    Success,
}

struct SetupWizard {
    current_step: WizardStep,
    username: String,
    potato_mode: bool,
    install_dcode: bool,
    selected_drive: String,
    progress: f32,
}

impl Default for SetupWizard {
    fn default() -> Self {
        Self {
            current_step: WizardStep::Welcome,
            username: String::from("AdminReaper9561"), // A legendary default starting point
            potato_mode: true,
            install_dcode: true,
            selected_drive: String::from("/dev/sda"),
            progress: 0.0,
        }
    }
}

impl eframe::App for SetupWizard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark()); // Premium telemetry-free dark space

        egui::CentralPanel::default().show(ctx, |ui| {
            // UI Header Zone
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("🪟 💨 DEFENESTRATION OS").size(24.0).strong());
                ui.label(egui::RichText::new("System Configuration Subsystem").italics());
                ui.add_space(10.0);
                ui.separator();
            });

            ui.add_space(15.0);

            // Step Logic Router
            match self.current_step {
                WizardStep::Welcome => self.render_welcome(ui),
                WizardStep::UserConfig => self.render_user_config(ui),
                WizardStep::TargetDisk => self.render_target_disk(ui),
                WizardStep::Installing => self.render_installing(ui, ctx),
                WizardStep::Success => self.render_success(ui),
            }
        });
    }
}

impl SetupWizard {
    fn render_welcome(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Welcome to a telemetry-free world.").size(16.0).strong());
        ui.add_space(10.0);
        ui.label("This wizard will prepare your machine by deploying an optimized, lightweight Linux overlay layout configured to salvage old hardware.");
        
        ui.add_space(20.0);
        ui.checkbox(&mut self.potato_mode, "Enable Aggressive RAM Saving (Potato Mode)");
        ui.checkbox(&mut self.install_dcode, "Pre-install d-code Text Editor");

        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            if ui.button("Next Step ➡️").clicked() {
                self.current_step = WizardStep::UserConfig;
            }
        });
    }

    fn render_user_config(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Profile Initialization").size(16.0).strong());
        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.label("Primary Username: ");
            ui.text_edit_singleline(&mut self.username);
            
            // The Unified Tag Roller Button
            if ui.button("🎲 Roll Tag").clicked() {
                self.username = generate_hacker_tag();
            }
        });

        ui.add_space(15.0);
        ui.label(egui::RichText::new("⚠️ Warning: Root password defaults to blank on live environment layout.")
            .color(egui::Color32::from_rgb(235, 160, 100)));

        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                if ui.button("⬅️ Back").clicked() { self.current_step = WizardStep::Welcome; }
                if ui.button("Next Step ➡️").clicked() { self.current_step = WizardStep::TargetDisk; }
            });
        });
    }

    fn render_target_disk(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Select Target Partition").size(16.0).strong());
        ui.add_space(15.0);

        egui::ComboBox::from_label("Target Local Storage")
            .selected_text(format!("{}", self.selected_drive))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected_drive, String::from("/dev/sda"), "/dev/sda (Local HDD)");
                ui.selectable_value(&mut self.selected_drive, String::from("/dev/nvme0n1"), "/dev/nvme0n1 (High-Speed SSD)");
            });

        ui.add_space(20.0);
        ui.label("Selecting a target drive will map the filesystem layers to this storage sector. Ensure all critical assets are backed up.");

        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                if ui.button("⬅️ Back").clicked() { self.current_step = WizardStep::UserConfig; }
                if ui.button("🚀 Finalize & Install").clicked() { self.current_step = WizardStep::Installing; }
            });
        });
    }

    fn render_installing(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label(egui::RichText::new("Writing System Blueprint to Disk...").size(16.0).strong());
        ui.add_space(20.0);

        self.progress += 0.004; // Smooth rolling load bar increments
        ui.add(egui::ProgressBar::new(self.progress).text(format!("{:.0}% Copied", self.progress * 100.0)));

        if self.progress >= 1.0 {
            self.current_step = WizardStep::Success;
        } else {
            ctx.request_repaint(); // Animation frame re-loop
        }
    }

    fn render_success(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(egui::RichText::new("🎉 ENVIRONMENT DEFENESTRATION COMPLETE").size(18.0).strong().color(egui::Color32::from_rgb(166, 227, 161)));
            ui.add_space(15.0);
            ui.label(format!("User profile '{}' successfully established. Windows telemetry pathways have been permanently severed.", self.username));
            ui.add_space(30.0);
            if ui.button("🔄 Restart into Defenestration OS").clicked() {
                std::process::exit(0);
            }
        });
    }
  }
