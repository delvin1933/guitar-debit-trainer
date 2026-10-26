use rand::seq::SliceRandom;
use rand::thread_rng;
use web_time::Instant;

use crate::audio::SoundEngine;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pattern {
    pub strokes: [bool; 4],
}

impl Pattern {
    pub fn all_3_note_patterns() -> Vec<Self> {
        vec![
            Pattern {
                strokes: [false, true, true, true],
            },
            Pattern {
                strokes: [true, false, true, true],
            },
            Pattern {
                strokes: [true, true, false, true],
            },
            Pattern {
                strokes: [true, true, true, false],
            },
        ]
    }

    pub fn display_string(&self) -> String {
        let labels = ["B", "H", "B", "H"];
        self.strokes
            .iter()
            .enumerate()
            .map(|(idx, &played)| if played { labels[idx] } else { "-" })
            .collect::<Vec<&str>>()
            .join("  ")
    }

    /// Génère l'affichage textuel du bloc complet répété N fois
    #[allow(dead_code)]
    pub fn display_block(&self, repetitions: u32) -> String {
        let single = self.display_string();
        std::iter::repeat_n(single, repetitions.max(1) as usize)
            .collect::<Vec<_>>()
            .join("   |   ")
    }
}

pub struct GuitarTrainerApp {
    pub bpm: u32,
    pub all_patterns: Vec<Pattern>,
    pub selected_patterns: Vec<bool>,
    pub repetitions: u32, // Nombre de répétitions par bloc (1 à 10)
    pub queue: Vec<Pattern>,
    pub is_running: bool,

    // Audio & Métrique
    audio: SoundEngine,
    last_tick: Instant,
    pub current_subdivision: usize, // 0 à 3 (les 4 doubles-croches du temps)
    pub current_repetition: u32,    // 0 à repetitions - 1

    // Réglages audio
    pub enable_click: bool,
    pub guitar_volume: f32,
    pub click_volume: f32,
}

impl GuitarTrainerApp {
    pub fn new() -> Self {
        let patterns = Pattern::all_3_note_patterns();
        let selected = vec![true; patterns.len()];
        let mut app = Self {
            bpm: 80,
            all_patterns: patterns,
            selected_patterns: selected,
            repetitions: 4,
            queue: Vec::new(),
            is_running: false,
            audio: SoundEngine::new(),
            last_tick: Instant::now(),
            current_subdivision: 0,
            current_repetition: 0,
            enable_click: true,
            guitar_volume: 0.8,
            click_volume: 0.6,
        };
        app.refill_queue();
        app
    }

    pub fn resume_audio(&self) {
        self.audio.resume();
    }

    pub fn refill_queue(&mut self) {
        let active_patterns: Vec<Pattern> = self
            .all_patterns
            .iter()
            .zip(self.selected_patterns.iter())
            .filter_map(|(p, &sel)| if sel { Some(*p) } else { None })
            .collect();

        while self.queue.len() < 3 {
            let next_pattern = if active_patterns.is_empty() {
                self.all_patterns[0]
            } else {
                *active_patterns.choose(&mut thread_rng()).unwrap()
            };
            self.queue.push(next_pattern);
        }
    }

    /// Avance le métronome et joue la note si nécessaire
    pub fn tick(&mut self) {
        if !self.is_running {
            return;
        }

        // Durée d'une double-croche en millisecondes : (60 000 ms / BPM) / 4
        let interval_ms = (60_000.0 / self.bpm as f64 / 4.0) as u64;

        if self.last_tick.elapsed().as_millis() >= interval_ms as u128 {
            self.last_tick = Instant::now();

            // Clic de métronome sur le début de chaque temps (subdivision 0)
            if self.current_subdivision == 0 && self.enable_click {
                self.audio.play_click(self.click_volume);
            }

            if let Some(current_pattern) = self.queue.first() {
                // Si la note doit être jouée sur cette double-croche
                if current_pattern.strokes[self.current_subdivision] {
                    self.audio.play_guitar_note(self.guitar_volume);
                }
            }

            // Incrémenter la double-croche
            self.current_subdivision += 1;

            // Si on a joué les 4 doubles-croches du temps (1 répétition)
            if self.current_subdivision >= 4 {
                self.current_subdivision = 0;
                self.current_repetition += 1;

                // Si on a complété le nombre de répétitions du bloc
                if self.current_repetition >= self.repetitions {
                    self.current_repetition = 0;
                    if !self.queue.is_empty() {
                        self.queue.remove(0);
                    }
                    self.refill_queue();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_creation_and_display() {
        let patterns = Pattern::all_3_note_patterns();
        assert_eq!(patterns.len(), 4);
        assert_eq!(patterns[0].display_string(), "-  H  B  H");
        assert_eq!(patterns[1].display_string(), "B  -  B  H");
        assert_eq!(patterns[2].display_string(), "B  H  -  H");
        assert_eq!(patterns[3].display_string(), "B  H  B  -");
    }

    #[test]
    fn test_pattern_display_block() {
        let p = Pattern {
            strokes: [true, false, true, true],
        };
        assert_eq!(p.display_block(1), "B  -  B  H");
        assert_eq!(
            p.display_block(3),
            "B  -  B  H   |   B  -  B  H   |   B  -  B  H"
        );
    }

    #[test]
    fn test_app_initialization() {
        let app = GuitarTrainerApp::new();
        assert_eq!(app.bpm, 80);
        assert_eq!(app.repetitions, 4);
        assert_eq!(app.current_repetition, 0);
        assert!(!app.is_running);
        assert!(app.enable_click);
        assert_eq!(app.guitar_volume, 0.8);
        assert_eq!(app.click_volume, 0.6);
        assert_eq!(app.current_subdivision, 0);
        assert_eq!(app.queue.len(), 3);
    }

    #[test]
    fn test_block_cycling() {
        let mut app = GuitarTrainerApp::new();
        app.is_running = true;
        app.repetitions = 2;
        let initial_pattern = app.queue[0];

        // 1ère répétition (4 subdivisions)
        for _ in 0..4 {
            app.last_tick = Instant::now() - std::time::Duration::from_millis(500);
            app.tick();
        }
        assert_eq!(app.current_repetition, 1);
        assert_eq!(app.queue[0], initial_pattern);

        // 2ème répétition (4 subdivisions) -> Fin du bloc, retour à la répétition 0
        for _ in 0..4 {
            app.last_tick = Instant::now() - std::time::Duration::from_millis(500);
            app.tick();
        }
        assert_eq!(app.current_repetition, 0);
    }
}
