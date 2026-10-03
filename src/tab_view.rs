use eframe::egui::{self, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use crate::app::Pattern;

/// Rendu graphique vectoriel de tablature rythmique conforme à ex_tab.png
pub fn draw_tablature(
    ui: &mut egui::Ui,
    pattern: &Pattern,
    repetitions: u32,
    current_rep: u32,
    current_sub: usize,
    is_running: bool,
    is_compact: bool,
) {
    let scale = if is_compact { 0.85 } else { 1.0 };
    let line_spacing = 9.0 * scale;
    let staff_height = 5.0 * line_spacing;
    let beat_width = 80.0 * scale;
    let header_width = 85.0 * scale;
    let padding_left = 15.0 * scale;
    let padding_top = 28.0 * scale; // Espace pour les coups de médiator B / H au-dessus
    let bottom_margin = 35.0 * scale; // Espace pour les hampes et doubles ligatures en dessous

    let total_beats = repetitions.max(1);
    let total_measures = (total_beats + 3) / 4;
    let total_width = padding_left + header_width + (total_beats as f32) * beat_width + (total_measures as f32) * (10.0 * scale) + 30.0 * scale;
    let total_height = padding_top + staff_height + bottom_margin;

    // Cadre type partition (papier blanc cassé / clair comme sur ex_tab.png)
    let (response, painter) = ui.allocate_painter(Vec2::new(total_width, total_height), egui::Sense::hover());
    let rect = response.rect;

    let paper_bg = Color32::from_rgb(250, 250, 252);
    let border_color = Color32::from_rgb(215, 215, 222);
    painter.rect_filled(rect, 6.0, paper_bg);
    painter.rect_stroke(rect, 6.0, Stroke::new(1.0_f32, border_color));

    let origin = Pos2::new(rect.min.x + padding_left, rect.min.y + padding_top);

    // Couleurs musicales
    let staff_line_color = Color32::from_rgb(70, 70, 75);
    let note_color = Color32::from_rgb(15, 15, 20);
    let active_color = Color32::from_rgb(230, 140, 0); // Orange / or vif pour la surbrillance active
    let active_bg = Color32::from_rgb(255, 238, 175); // Halo jaune doux
    let stroke_text_color = Color32::from_rgb(60, 60, 70);

    // 1. Tracé des 6 lignes de portée (e, B, G, D, A, E)
    let staff_end_x = rect.max.x - 15.0 * scale;
    for i in 0..6 {
        let y = origin.y + (i as f32) * line_spacing;
        painter.line_segment(
            [Pos2::new(origin.x, y), Pos2::new(staff_end_x, y)],
            Stroke::new(1.0_f32, staff_line_color),
        );
    }

    // 2. Clé de Sol et Signature 4/4
    let clef_x = origin.x + 12.0 * scale;
    // Clé de sol stylisée
    painter.text(
        Pos2::new(clef_x, origin.y + line_spacing * 2.5),
        egui::Align2::CENTER_CENTER,
        "𝄞",
        FontId::proportional(34.0 * scale),
        note_color,
    );
    // Petit 8 d'octave sous la clé de sol
    painter.text(
        Pos2::new(clef_x, origin.y + staff_height + 5.0 * scale),
        egui::Align2::CENTER_CENTER,
        "8",
        FontId::proportional(10.0 * scale),
        note_color,
    );

    // Signature 4/4
    let time_sig_x = clef_x + 24.0 * scale;
    painter.text(
        Pos2::new(time_sig_x, origin.y + line_spacing * 1.3),
        egui::Align2::CENTER_CENTER,
        "4",
        FontId::monospace(18.0 * scale),
        note_color,
    );
    painter.text(
        Pos2::new(time_sig_x, origin.y + line_spacing * 3.7),
        egui::Align2::CENTER_CENTER,
        "4",
        FontId::monospace(18.0 * scale),
        note_color,
    );

    // 3. Barre de reprise début |:
    let repeat_bar_x = time_sig_x + 22.0 * scale;
    // Barre épaisse
    painter.line_segment(
        [Pos2::new(repeat_bar_x, origin.y), Pos2::new(repeat_bar_x, origin.y + staff_height)],
        Stroke::new(3.0 * scale, note_color),
    );
    // Barre fine
    painter.line_segment(
        [Pos2::new(repeat_bar_x + 4.0 * scale, origin.y), Pos2::new(repeat_bar_x + 4.0 * scale, origin.y + staff_height)],
        Stroke::new(1.0_f32, note_color),
    );
    // Deux points de reprise
    let dots_x = repeat_bar_x + 10.0 * scale;
    painter.circle_filled(Pos2::new(dots_x, origin.y + line_spacing * 1.5), 2.2 * scale, note_color);
    painter.circle_filled(Pos2::new(dots_x, origin.y + line_spacing * 3.5), 2.2 * scale, note_color);

    // 4. Tracé des temps (battements)
    let notes_start_x = dots_x + 16.0 * scale;
    let d_string_y = origin.y + 3.0 * line_spacing; // 4ème ligne = Corde de Ré
    let stem_bottom_y = origin.y + staff_height + 18.0 * scale; // Bas des hampes

    let mut current_x = notes_start_x;

    for rep in 0..repetitions {
        // Barre de mesure tous les 4 temps (mesure 4/4)
        if rep > 0 && rep % 4 == 0 {
            painter.line_segment(
                [Pos2::new(current_x - 4.0 * scale, origin.y), Pos2::new(current_x - 4.0 * scale, origin.y + staff_height)],
                Stroke::new(1.2 * scale, staff_line_color),
            );
            current_x += 12.0 * scale;
        }

        let is_current_rep = is_running && rep == current_rep;
        let beat_sub_width = beat_width / 4.0;
        let mut played_x_positions = Vec::new();

        // Labels des coups de médiator pour les 4 subdivisions
        let stroke_labels = ["B", "H", "B", "H"];

        for sub in 0..4 {
            let note_x = current_x + (sub as f32 + 0.5) * beat_sub_width;
            let is_played = pattern.strokes[sub];
            let is_active_sub = is_current_rep && sub == current_sub;

            // Coup de médiator B / H au-dessus de la portée
            if is_played {
                let stroke_color = if is_active_sub { active_color } else { stroke_text_color };
                let stroke_font_size = if is_active_sub { 15.0 * scale } else { 13.0 * scale };
                painter.text(
                    Pos2::new(note_x, origin.y - 12.0 * scale),
                    egui::Align2::CENTER_CENTER,
                    stroke_labels[sub],
                    FontId::monospace(stroke_font_size),
                    stroke_color,
                );
            }

            if is_played {
                played_x_positions.push(note_x);

                // Halo lumineux sur la note active
                if is_active_sub {
                    painter.circle_filled(Pos2::new(note_x, d_string_y), 9.0 * scale, active_bg);
                }

                // Masque blanc derrière le chiffre '5' pour ne pas couper le chiffre avec la ligne
                let mask_w = 12.0 * scale;
                let mask_h = 10.0 * scale;
                let mask_rect = Rect::from_center_size(Pos2::new(note_x, d_string_y), Vec2::new(mask_w, mask_h));
                painter.rect_filled(mask_rect, 2.0, if is_active_sub { active_bg } else { paper_bg });

                // Chiffre '5' (Sol sur la corde de Ré)
                let n_color = if is_active_sub { active_color } else { note_color };
                painter.text(
                    Pos2::new(note_x, d_string_y),
                    egui::Align2::CENTER_CENTER,
                    "5",
                    FontId::monospace(14.0 * scale),
                    n_color,
                );

                // Hampe verticale vers le bas
                let stem_color = if is_active_sub { active_color } else { note_color };
                painter.line_segment(
                    [Pos2::new(note_x, d_string_y + 5.0 * scale), Pos2::new(note_x, stem_bottom_y)],
                    Stroke::new(1.3 * scale, stem_color),
                );
            } else {
                // Silence de double-croche (quart de soupir / 16th rest)
                let rest_color = if is_active_sub { active_color } else { Color32::from_rgb(90, 90, 100) };
                let rest_y = origin.y + line_spacing * 2.8;

                // Halo sur le silence actif
                if is_active_sub {
                    painter.circle_filled(Pos2::new(note_x, rest_y + 4.0 * scale), 9.0 * scale, active_bg);
                }

                // Tracé vectoriel du quart de soupir
                // Hampe oblique
                painter.line_segment(
                    [Pos2::new(note_x + 2.0 * scale, rest_y - 2.0 * scale), Pos2::new(note_x - 3.0 * scale, rest_y + 12.0 * scale)],
                    Stroke::new(1.4 * scale, rest_color),
                );
                // 1er crochet
                painter.line_segment(
                    [Pos2::new(note_x - 3.0 * scale, rest_y + 1.0 * scale), Pos2::new(note_x + 1.5 * scale, rest_y + 1.0 * scale)],
                    Stroke::new(1.8 * scale, rest_color),
                );
                // 2ème crochet
                painter.line_segment(
                    [Pos2::new(note_x - 4.5 * scale, rest_y + 6.0 * scale), Pos2::new(note_x - 0.5 * scale, rest_y + 6.0 * scale)],
                    Stroke::new(1.8 * scale, rest_color),
                );
            }
        }

        // 5. Tracé des doubles ligatures (beams) pour les notes consécutives du temps
        draw_beams(
            &painter,
            &pattern.strokes,
            current_x,
            beat_sub_width,
            stem_bottom_y,
            scale,
            is_current_rep,
            current_sub,
            note_color,
            active_color,
        );

        // Barre fine de séparation de temps
        let beat_end_x = current_x + beat_width;
        painter.line_segment(
            [Pos2::new(beat_end_x, origin.y), Pos2::new(beat_end_x, origin.y + staff_height)],
            Stroke::new(0.6 * scale, Color32::from_rgb(200, 200, 205)),
        );

        current_x += beat_width;
    }

    // 6. Barre de fin de reprise :|
    painter.line_segment(
        [Pos2::new(current_x + 4.0 * scale, origin.y), Pos2::new(current_x + 4.0 * scale, origin.y + staff_height)],
        Stroke::new(1.0_f32, note_color),
    );
    painter.line_segment(
        [Pos2::new(current_x + 8.0 * scale, origin.y), Pos2::new(current_x + 8.0 * scale, origin.y + staff_height)],
        Stroke::new(3.0 * scale, note_color),
    );
    painter.circle_filled(Pos2::new(current_x - 2.0 * scale, origin.y + line_spacing * 1.5), 2.2 * scale, note_color);
    painter.circle_filled(Pos2::new(current_x - 2.0 * scale, origin.y + line_spacing * 3.5), 2.2 * scale, note_color);
}

/// Trace les doubles ligatures (beams) pour les 4 subdivisions d'un temps
fn draw_beams(
    painter: &egui::Painter,
    strokes: &[bool; 4],
    beat_start_x: f32,
    sub_w: f32,
    stem_bottom_y: f32,
    scale: f32,
    is_current_rep: bool,
    current_sub: usize,
    note_color: Color32,
    active_color: Color32,
) {
    let beam1_y = stem_bottom_y;
    let beam2_y = stem_bottom_y - 4.5 * scale;
    let beam_thickness = 2.4 * scale;

    let sub_x = |sub: usize| beat_start_x + (sub as f32 + 0.5) * sub_w;

    // Chercher les blocs consécutifs de notes jouées
    let mut i = 0;
    while i < 4 {
        if strokes[i] {
            let start = i;
            while i < 4 && strokes[i] {
                i += 1;
            }
            let end = i - 1;

            if start < end {
                // Plusieurs notes consécutives : on les relie par une double ligature continue
                let x_start = sub_x(start);
                let x_end = sub_x(end);

                let is_active = is_current_rep && (current_sub >= start && current_sub <= end);
                let b_color = if is_active { active_color } else { note_color };

                painter.line_segment([Pos2::new(x_start, beam1_y), Pos2::new(x_end, beam1_y)], Stroke::new(beam_thickness, b_color));
                painter.line_segment([Pos2::new(x_start, beam2_y), Pos2::new(x_end, beam2_y)], Stroke::new(beam_thickness, b_color));
            } else {
                // Note isolée : petit crochet double orienté vers la droite ou vers la note voisine
                let x = sub_x(start);
                let hook_len = 6.0 * scale;
                let hook_dir = if start == 3 { -hook_len } else { hook_len };

                let is_active = is_current_rep && current_sub == start;
                let b_color = if is_active { active_color } else { note_color };

                painter.line_segment([Pos2::new(x, beam1_y), Pos2::new(x + hook_dir, beam1_y)], Stroke::new(beam_thickness, b_color));
                painter.line_segment([Pos2::new(x, beam2_y), Pos2::new(x + hook_dir, beam2_y)], Stroke::new(beam_thickness, b_color));
            }
        } else {
            i += 1;
        }
    }
}
