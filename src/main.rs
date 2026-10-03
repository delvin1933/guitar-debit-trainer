use eframe::egui;
mod app;
mod audio;
mod tab_view;
use app::GuitarTrainerApp;

impl eframe::App for GuitarTrainerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Faire avancer le métronome/audio
        self.tick();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.heading("Entraînement au Débit Guitare (3 notes sur 4)");
                ui.add_space(6.0);

                // Bouton START / STOP, Tempo (BPM) et Répétitions par bloc
                ui.horizontal(|ui| {
                    let start_btn_text = if self.is_running { "⏸ PAUSE" } else { "▶ START" };
                    let btn_color = if self.is_running {
                        egui::Color32::from_rgb(200, 60, 60)
                    } else {
                        egui::Color32::from_rgb(60, 180, 60)
                    };

                    if ui.add_sized([100.0, 30.0], egui::Button::new(start_btn_text).fill(btn_color)).clicked() {
                        self.is_running = !self.is_running;
                        self.current_subdivision = 0;
                        self.current_repetition = 0;
                    }

                    ui.add_space(15.0);
                    ui.label("Tempo (BPM) :");
                    ui.add(egui::Slider::new(&mut self.bpm, 40..=220));

                    ui.add_space(20.0);
                    let prev_reps = self.repetitions;
                    egui::ComboBox::from_label("Répétitions par bloc")
                        .selected_text(format!("{} fois", self.repetitions))
                        .show_ui(ui, |ui| {
                            for n in 1..=10 {
                                ui.selectable_value(&mut self.repetitions, n, format!("{} fois", n));
                            }
                        });
                    if self.repetitions != prev_reps && self.current_repetition >= self.repetitions {
                        self.current_repetition = 0;
                    }
                });

                ui.add_space(4.0);

                // Réglages audio (Clic & Volumes)
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.enable_click, "🔔 Clic métronome");
                    ui.add_space(15.0);
                    ui.label("Vol. Guitare :");
                    ui.add(egui::Slider::new(&mut self.guitar_volume, 0.0..=1.0).show_value(false));
                    ui.add_space(10.0);
                    ui.label("Vol. Clic :");
                    ui.add(egui::Slider::new(&mut self.click_volume, 0.0..=1.0).show_value(false));
                });

                ui.separator();

                // Sélection des variations incluses
                ui.label(egui::RichText::new("Variations à inclure :").strong());
                ui.horizontal(|ui| {
                    let mut changed = false;
                    for idx in 0..self.all_patterns.len() {
                        let label = self.all_patterns[idx].display_string();
                        if ui.checkbox(&mut self.selected_patterns[idx], label).changed() {
                            changed = true;
                        }
                    }
                    if changed {
                        self.queue.clear();
                        self.refill_queue();
                        self.current_repetition = 0;
                    }
                });

                ui.separator();

                // Affichage Tablature du bloc actuel
                let rep_info = if self.is_running {
                    format!("Bloc actuel (Répétition {} / {}) :", self.current_repetition + 1, self.repetitions)
                } else {
                    format!("Bloc actuel ({} répétitions) :", self.repetitions)
                };
                ui.label(egui::RichText::new(rep_info).size(18.0).strong());
                ui.add_space(4.0);

                if let Some(current_pattern) = self.queue.first() {
                    tab_view::draw_tablature(
                        ui,
                        current_pattern,
                        self.repetitions,
                        self.current_repetition,
                        self.current_subdivision,
                        self.is_running,
                        false,
                    );
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(4.0);

                // Blocs suivants sous forme de tablature
                ui.label(egui::RichText::new("Blocs suivants :").size(16.0).strong());
                ui.add_space(4.0);

                if self.queue.len() > 1 {
                    ui.label(egui::RichText::new("1.").size(14.0).strong());
                    tab_view::draw_tablature(
                        ui,
                        &self.queue[1],
                        self.repetitions,
                        0,
                        0,
                        false,
                        true,
                    );
                }
                ui.add_space(6.0);
                if self.queue.len() > 2 {
                    ui.label(egui::RichText::new("2.").size(14.0).strong());
                    tab_view::draw_tablature(
                        ui,
                        &self.queue[2],
                        self.repetitions,
                        0,
                        0,
                        false,
                        true,
                    );
                }
            });
        });

        // Demande constante de rafraîchissement pour alimenter le métronome
        ctx.request_repaint();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(egui::vec2(1020.0, 680.0));
    options.viewport.min_inner_size = Some(egui::vec2(750.0, 480.0));
    eframe::run_native(
        "Débit Guitare",
        options,
        Box::new(|_| Box::new(GuitarTrainerApp::new())),
    )
}
