use eframe::egui::{self, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use crate::app::Pattern;

/// Rendu graphique vectoriel et 100% réactif de tablature rythmique conforme à ex_tab.png
pub fn draw_tablature(
    ui: &mut egui::Ui,
    pattern: &Pattern,
    repetitions: u32,
    current_rep: u32,
    current_sub: usize,
    is_running: bool,
    is_compact: bool,
) {
    let scale = if is_compact { 0.85_f32 } else { 1.0_f32 };
    let line_spacing = 9.0_f32 * scale;
    let staff_height = 5.0_f32 * line_spacing;
    let padding_left = 14.0_f32 * scale;
    let padding_top = 28.0_f32 * scale; // Espace pour les coups de médiator B / H
    let bottom_margin = 34.0_f32 * scale; // Espace pour les hampes et doubles ligatures
    let line_staff_height = padding_top + staff_height + bottom_margin;
    let line_gap = 14.0_f32 * scale;

    let available_w = ui.available_width().max(320.0_f32);
    let total_beats = repetitions.max(1);

    // Largeur minimale confortable par temps (pour éviter que les notes se chevauchent)
    let min_beat_w = 70.0_f32 * scale;
    let full_header_w = 84.0_f32 * scale;
    let continuation_header_w = 44.0_f32 * scale;
    let end_bar_w = 20.0_f32 * scale;

    // Calcul réactif : est-ce que tous les temps tiennent sur 1 seule ligne fluide ?
    let single_line_needed = full_header_w + (total_beats as f32) * min_beat_w + end_bar_w + padding_left * 2.0_f32;
    let beats_per_line = if total_beats <= 4 {
        total_beats
    } else if available_w >= single_line_needed {
        total_beats
    } else {
        4 // Découpage responsive par mesure de 4/4
    };

    let num_lines = ((total_beats + beats_per_line - 1) / beats_per_line) as usize;
    let total_height = (num_lines as f32) * line_staff_height + ((num_lines.saturating_sub(1)) as f32) * line_gap;

    // Allocation de l'espace peintre étiré sur 100% de la largeur disponible
    let (response, painter) = ui.allocate_painter(Vec2::new(available_w, total_height), egui::Sense::hover());
    let rect = response.rect;

    // Fond partition papier blanc/ivoire propre conforme à ex_tab.png
    let paper_bg = Color32::from_rgb(250, 250, 252);
    let border_color = Color32::from_rgb(215, 215, 222);
    painter.rect_filled(rect, 6.0_f32, paper_bg);
    painter.rect_stroke(rect, 6.0_f32, Stroke::new(1.0_f32, border_color));

    // Couleurs musicales
    let staff_line_color = Color32::from_rgb(70, 70, 75);
    let note_color = Color32::from_rgb(15, 15, 20);
    let active_color = Color32::from_rgb(230, 140, 0); // Orange / or pour le curseur actif
    let active_bg = Color32::from_rgb(255, 238, 175); // Halo jaune lumineux
    let stroke_text_color = Color32::from_rgb(60, 60, 70);

    let stroke_labels = ["B", "H", "B", "H"];

    // Dessin de chaque système (ligne de portée)
    for line_idx in 0..num_lines {
        let start_beat = (line_idx as u32) * beats_per_line;
        let end_beat = (start_beat + beats_per_line).min(total_beats);
        let line_beats_count = (end_beat - start_beat) as f32;

        let is_first_line = line_idx == 0;
        let is_last_line = line_idx == num_lines - 1;

        let y_top = rect.min.y + (line_idx as f32) * (line_staff_height + line_gap) + padding_top;
        let staff_start_x = rect.min.x + padding_left;
        let staff_end_x = rect.max.x - padding_left;
        let line_width = staff_end_x - staff_start_x;

        // 1. Tracé des 6 lignes de portée
        for s in 0..6 {
            let y = y_top + (s as f32) * line_spacing;
            painter.line_segment(
                [Pos2::new(staff_start_x, y), Pos2::new(staff_end_x, y)],
                Stroke::new(1.0_f32, staff_line_color),
            );
        }

        // 2. En-tête de ligne (Clé de sol et métrique)
        let (header_w, notes_start_x) = if is_first_line {
            let clef_x = staff_start_x + 12.0_f32 * scale;
            // Clé de sol
            painter.text(
                Pos2::new(clef_x, y_top + line_spacing * 2.5_f32),
                egui::Align2::CENTER_CENTER,
                "𝄞",
                FontId::proportional(34.0_f32 * scale),
                note_color,
            );
            // Petit 8 d'octave sous la clé
            painter.text(
                Pos2::new(clef_x, y_top + staff_height + 5.0_f32 * scale),
                egui::Align2::CENTER_CENTER,
                "8",
                FontId::proportional(10.0_f32 * scale),
                note_color,
            );

            // Signature 4/4
            let time_sig_x = clef_x + 22.0_f32 * scale;
            painter.text(
                Pos2::new(time_sig_x, y_top + line_spacing * 1.3_f32),
                egui::Align2::CENTER_CENTER,
                "4",
                FontId::monospace(18.0_f32 * scale),
                note_color,
            );
            painter.text(
                Pos2::new(time_sig_x, y_top + line_spacing * 3.7_f32),
                egui::Align2::CENTER_CENTER,
                "4",
                FontId::monospace(18.0_f32 * scale),
                note_color,
            );

            // Barre de reprise début |:
            let rep_bar_x = time_sig_x + 20.0_f32 * scale;
            painter.line_segment(
                [Pos2::new(rep_bar_x, y_top), Pos2::new(rep_bar_x, y_top + staff_height)],
                Stroke::new(3.0_f32 * scale, note_color),
            );
            painter.line_segment(
                [Pos2::new(rep_bar_x + 4.0_f32 * scale, y_top), Pos2::new(rep_bar_x + 4.0_f32 * scale, y_top + staff_height)],
                Stroke::new(1.0_f32 * scale, note_color),
            );
            let dots_x = rep_bar_x + 9.0_f32 * scale;
            painter.circle_filled(Pos2::new(dots_x, y_top + line_spacing * 1.5_f32), 2.2_f32 * scale, note_color);
            painter.circle_filled(Pos2::new(dots_x, y_top + line_spacing * 3.5_f32), 2.2_f32 * scale, note_color);

            (full_header_w, staff_start_x + full_header_w)
        } else {
            // Ligne de continuation : clé de sol simplifiée
            let clef_x = staff_start_x + 12.0_f32 * scale;
            painter.text(
                Pos2::new(clef_x, y_top + line_spacing * 2.5_f32),
                egui::Align2::CENTER_CENTER,
                "𝄞",
                FontId::proportional(28.0_f32 * scale),
                note_color,
            );
            let bar_x = clef_x + 16.0_f32 * scale;
            painter.line_segment(
                [Pos2::new(bar_x, y_top), Pos2::new(bar_x, y_top + staff_height)],
                Stroke::new(1.5_f32 * scale, note_color),
            );
            (continuation_header_w, staff_start_x + continuation_header_w)
        };

        // 3. Calcul fluide de la largeur par temps pour remplir 100% de la ligne
        let end_decor_w = if is_last_line { end_bar_w } else { 6.0_f32 * scale };
        let usable_width = (line_width - header_w - end_decor_w).max(line_beats_count * min_beat_w);
        let dynamic_beat_w = usable_width / line_beats_count;

        let d_string_y = y_top + 3.0_f32 * line_spacing; // 4ème ligne = Corde de Ré
        let stem_bottom_y = y_top + staff_height + 18.0_f32 * scale;

        let mut current_x = notes_start_x;

        for rep in start_beat..end_beat {
            // Barre de mesure tous les 4 temps (mesure de 4/4)
            if rep > start_beat && rep % 4 == 0 {
                painter.line_segment(
                    [Pos2::new(current_x - 3.0_f32 * scale, y_top), Pos2::new(current_x - 3.0_f32 * scale, y_top + staff_height)],
                    Stroke::new(1.3_f32 * scale, staff_line_color),
                );
            }

            let is_current_rep = is_running && rep == current_rep;
            let sub_w = dynamic_beat_w / 4.0_f32;

            for sub in 0..4 {
                let note_x = current_x + (sub as f32 + 0.5_f32) * sub_w;
                let is_played = pattern.strokes[sub];
                let is_active_sub = is_current_rep && sub == current_sub;

                // Coup de médiator B / H au-dessus de la portée
                if is_played {
                    let stroke_color = if is_active_sub { active_color } else { stroke_text_color };
                    let stroke_font_size = if is_active_sub { 15.0_f32 * scale } else { 13.0_f32 * scale };
                    painter.text(
                        Pos2::new(note_x, y_top - 12.0_f32 * scale),
                        egui::Align2::CENTER_CENTER,
                        stroke_labels[sub],
                        FontId::monospace(stroke_font_size),
                        stroke_color,
                    );
                }

                if is_played {
                    // Halo lumineux sur la note active
                    if is_active_sub {
                        painter.circle_filled(Pos2::new(note_x, d_string_y), 9.0_f32 * scale, active_bg);
                    }

                    // Masque propre derrière le chiffre '5'
                    let mask_w = 12.0_f32 * scale;
                    let mask_h = 10.0_f32 * scale;
                    let mask_rect = Rect::from_center_size(Pos2::new(note_x, d_string_y), Vec2::new(mask_w, mask_h));
                    painter.rect_filled(mask_rect, 2.0_f32, if is_active_sub { active_bg } else { paper_bg });

                    // Chiffre '5' (Sol sur corde de Ré)
                    let n_color = if is_active_sub { active_color } else { note_color };
                    painter.text(
                        Pos2::new(note_x, d_string_y),
                        egui::Align2::CENTER_CENTER,
                        "5",
                        FontId::monospace(14.0_f32 * scale),
                        n_color,
                    );

                    // Hampe verticale vers le bas
                    let stem_color = if is_active_sub { active_color } else { note_color };
                    painter.line_segment(
                        [Pos2::new(note_x, d_string_y + 5.0_f32 * scale), Pos2::new(note_x, stem_bottom_y)],
                        Stroke::new(1.3_f32 * scale, stem_color),
                    );
                } else {
                    // Silence de double-croche (quart de soupir / 16th rest)
                    let rest_color = if is_active_sub { active_color } else { Color32::from_rgb(90, 90, 100) };
                    let rest_y = y_top + line_spacing * 2.8_f32;

                    if is_active_sub {
                        painter.circle_filled(Pos2::new(note_x, rest_y + 4.0_f32 * scale), 9.0_f32 * scale, active_bg);
                    }

                    // Tracé vectoriel du quart de soupir
                    painter.line_segment(
                        [Pos2::new(note_x + 2.0_f32 * scale, rest_y - 2.0_f32 * scale), Pos2::new(note_x - 3.0_f32 * scale, rest_y + 12.0_f32 * scale)],
                        Stroke::new(1.4_f32 * scale, rest_color),
                    );
                    painter.line_segment(
                        [Pos2::new(note_x - 3.0_f32 * scale, rest_y + 1.0_f32 * scale), Pos2::new(note_x + 1.5_f32 * scale, rest_y + 1.0_f32 * scale)],
                        Stroke::new(1.8_f32 * scale, rest_color),
                    );
                    painter.line_segment(
                        [Pos2::new(note_x - 4.5_f32 * scale, rest_y + 6.0_f32 * scale), Pos2::new(note_x - 0.5_f32 * scale, rest_y + 6.0_f32 * scale)],
                        Stroke::new(1.8_f32 * scale, rest_color),
                    );
                }
            }

            // Tracé des doubles ligatures (beams) pour les notes consécutives du temps
            draw_beams(
                &painter,
                &pattern.strokes,
                current_x,
                sub_w,
                stem_bottom_y,
                scale,
                is_current_rep,
                current_sub,
                note_color,
                active_color,
            );

            // Séparateur de temps discret
            let beat_end_x = current_x + dynamic_beat_w;
            painter.line_segment(
                [Pos2::new(beat_end_x, y_top), Pos2::new(beat_end_x, y_top + staff_height)],
                Stroke::new(0.6_f32 * scale, Color32::from_rgb(205, 205, 210)),
            );

            current_x += dynamic_beat_w;
        }

        // 4. Barre de fin de mesure ou de reprise :|
        if is_last_line {
            painter.line_segment(
                [Pos2::new(staff_end_x - 6.0_f32 * scale, y_top), Pos2::new(staff_end_x - 6.0_f32 * scale, y_top + staff_height)],
                Stroke::new(1.0_f32, note_color),
            );
            painter.line_segment(
                [Pos2::new(staff_end_x - 2.0_f32 * scale, y_top), Pos2::new(staff_end_x - 2.0_f32 * scale, y_top + staff_height)],
                Stroke::new(3.0_f32 * scale, note_color),
            );
            painter.circle_filled(Pos2::new(staff_end_x - 12.0_f32 * scale, y_top + line_spacing * 1.5_f32), 2.2_f32 * scale, note_color);
            painter.circle_filled(Pos2::new(staff_end_x - 12.0_f32 * scale, y_top + line_spacing * 3.5_f32), 2.2_f32 * scale, note_color);
        } else {
            painter.line_segment(
                [Pos2::new(staff_end_x - 2.0_f32 * scale, y_top), Pos2::new(staff_end_x - 2.0_f32 * scale, y_top + staff_height)],
                Stroke::new(1.5_f32 * scale, staff_line_color),
            );
        }
    }
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
    let beam2_y = stem_bottom_y - 4.5_f32 * scale;
    let beam_thickness = 2.4_f32 * scale;

    let sub_x = |sub: usize| beat_start_x + (sub as f32 + 0.5_f32) * sub_w;

    let mut i = 0;
    while i < 4 {
        if strokes[i] {
            let start = i;
            while i < 4 && strokes[i] {
                i += 1;
            }
            let end = i - 1;

            if start < end {
                // Plusieurs notes consécutives : double ligature continue
                let x_start = sub_x(start);
                let x_end = sub_x(end);

                let is_active = is_current_rep && (current_sub >= start && current_sub <= end);
                let b_color = if is_active { active_color } else { note_color };

                painter.line_segment([Pos2::new(x_start, beam1_y), Pos2::new(x_end, beam1_y)], Stroke::new(beam_thickness, b_color));
                painter.line_segment([Pos2::new(x_start, beam2_y), Pos2::new(x_end, beam2_y)], Stroke::new(beam_thickness, b_color));
            } else {
                // Note isolée : petit crochet double vers la droite (ou gauche pour la dernière)
                let x = sub_x(start);
                let hook_len = 6.0_f32 * scale;
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
