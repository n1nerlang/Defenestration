use eframe::egui;
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::str;

// ============================================================================
// 🧠 Core Search Engine Logic (Pure no_std / alloc compatible)
// ============================================================================

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Posting {
    pub doc_id: u32,
    pub term_frequency: u32,
}

#[derive(Default)]
pub struct InvertedIndex {
    pub lexicon: BTreeMap<String, Vec<Posting>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self { lexicon: BTreeMap::new() }
    }

    pub fn index_document(&mut self, doc_id: u32, raw_bytes: &[u8]) {
        let tokens = tokenize(raw_bytes);
        for token in tokens {
            let postings = self.lexicon.entry(token).or_insert_with(Vec::new);
            if let Some(posting) = postings.iter_mut().find(|p| p.doc_id == doc_id) {
                posting.term_frequency += 1;
            } else {
                postings.push(Posting { doc_id, term_frequency: 1 });
            }
        }
    }

    pub fn search(&self, query_term: &str) -> Vec<Posting> {
        let mut normalized_query = String::new();
        for b in query_term.as_bytes() {
            if b.is_ascii_alphanumeric() {
                normalized_query.push(b.to_ascii_lowercase() as char);
            }
        }

        if let Some(postings) = self.lexicon.get(&normalized_query) {
            let mut ranked = postings.clone();
            // Sort descending: highest term frequency hits the top
            ranked.sort_by(|a, b| b.term_frequency.cmp(&a.term_frequency));
            ranked
        } else {
            Vec::new()
        }
    }
}

pub fn tokenize(bytes: &[u8]) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = Vec::new();

    for &b in bytes {
        if b.is_ascii_alphanumeric() {
            current_token.push(b.to_ascii_lowercase());
        } else if !current_token.is_empty() {
            if let Ok(valid_str) = str::from_utf8(&current_token) {
                tokens.push(String::from(valid_str));
            }
            current_token.clear();
        }
    }
    if !current_token.is_empty() {
        if let Ok(valid_str) = str::from_utf8(&current_token) {
            tokens.push(String::from(valid_str));
        }
    }
    tokens
}

// Simulated VFS Layer
pub struct VfsFile {
    pub id: u32,
    pub path: &'static str,
    pub payload: &'static str,
}

// ============================================================================
// 🎨 Desktop UI Layer (egui Integration)
// ============================================================================

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([650.0, 500.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "defenestration_search",
        options,
        Box::new(|_cc| Ok(Box::new(SearchApp::new()))),
    )
}

struct SearchApp {
    index: InvertedIndex,
    vfs_manifest: Vec<VfsFile>,
    search_query: String,
    search_results: Vec<Posting>,
}

impl SearchApp {
    fn new() -> Self {
        let mut index = InvertedIndex::new();
        
        // Load up sample OS files into our mock memory storage pointers
        let vfs_manifest = vec![
            VfsFile { id: 101, path: "/sys/kernel/shield.log", payload: "Telemetry warning: Microsoft connection intercepted and blocked. Pure performance." },
            VfsFile { id: 102, path: "/user/documents/notes.txt", payload: "Defenestration OS is ridiculously fast. Pure speed, zero tracking bloat, optimized kernel parameters." },
            VfsFile { id: 103, path: "/etc/hosts", payload: "127.0.0.1 telemetry.microsoft.com\n127.0.0.1 vortex.data.microsoft.com" },
            VfsFile { id: 104, path: "/sys/drivers/gpu.sys", payload: "Graphics acceleration activated. High speed memory pipelines mapping frames perfectly." },
        ];

        // Process and index everything right at startup
        for file in &vfs_manifest {
            index.index_document(file.id, file.payload.as_bytes());
        }

        Self {
            index,
            vfs_manifest,
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }
}

impl eframe::App for SearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header Section
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("🔍 DEFENESTRATION SEARCH ENGINE").size(22.0).strong());
                ui.label(egui::RichText::new("Bare-metal layout indexing with zero syscall overhead").italics().color(egui::Color32::GRAY));
                ui.add_space(10.0);
                ui.separator();
            });

            ui.add_space(15.0);

            // Live Search Bar Input Loop
            ui.label(egui::RichText::new("Enter lookup term:").strong());
            ui.horizontal(|ui| {
                let text_edit = ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Type to search (e.g., telemetry, speed, kernel)...")
                        .desired_width(ui.available_width() - 90.0)
                );

                // Run search instantaneously when the text updates or user hits enter
                if text_edit.changed() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if self.search_query.trim().is_empty() {
                        self.search_results.clear();
                    } else {
                        self.search_results = self.index.search(&self.search_query);
                    }
                }

                if ui.button("Clear 🗑️").clicked() {
                    self.search_query.clear();
                    self.search_results.clear();
                }
            });

            ui.add_space(20.0);

            // Main Content Layout Split: Results vs Indexed Files Catalog
            ui.columns(2, |columns| {
                // Column 1: Active Ranked Results
                columns[0].vertical(|ui| {
                    ui.label(egui::RichText::new("Ranked Hits (By Term Frequency)").underline().color(egui::Color32::LIGHT_BLUE));
                    ui.add_space(5.0);

                    if self.search_query.trim().is_empty() {
                        ui.label("Waiting for search queries...");
                    } else if self.search_results.is_empty() {
                        ui.label("❌ No occurrences found in memory layout.");
                    } else {
                        egui::ScrollArea::vertical().id_source("results_scroll").show(ui, |ui| {
                            for posting in &self.search_results {
                                if let Some(file) = self.vfs_manifest.iter().find(|f| f.id == posting.doc_id) {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(file.path).strong().color(egui::Color32::WHITE));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(egui::RichText::new(format!("Hits: {}", posting.term_frequency))
                                                    .background_color(egui::Color32::from_rgb(40, 80, 40))
                                                    .color(egui::Color32::GREEN));
                                            });
                                        });
                                        ui.small(format!("ID: {} | Snippet: \"{}...\"", file.id, &file.payload[..core::cmp::min(file.payload.len(), 45)]));
                                    });
                                    ui.add_space(4.0);
                                }
                            }
                        });
                    }
                });

                // Column 2: System VFS Inspector Panel
                columns[1].vertical(|ui| {
                    ui.label(egui::RichText::new("Target Filesystem Registry").underline().color(egui::Color32::GRAY));
                    ui.add_space(5.0);

                    egui::ScrollArea::vertical().id_source("vfs_scroll").show(ui, |ui| {
                        for file in &self.vfs_manifest {
                            ui.group(|ui| {
                                ui.label(egui::RichText::new(file.path).small().color(egui::Color32::LIGHT_GRAY));
                                ui.label(egui::RichText::new(format!("Size: {} bytes", file.payload.len())).weak().small());
                            });
                            ui.add_space(3.0);
                        }
                    });
                });
            });
        });
    }
}
