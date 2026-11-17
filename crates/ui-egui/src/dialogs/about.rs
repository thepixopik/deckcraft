use super::{Color32, CornerRadius, Rect, Sense, SlideApp, Stroke, Tokens, Ui, pos2, theme, vec2};

/// Room the About credits need for their table; the About tab keeps its compact size.
pub(super) const CREDITS_SIZE: egui::Vec2 = vec2(820.0, 380.0);

/// The Contributors / Models body in a block of its own fixed size, below the tab row.
pub(super) fn credits_block(ui: &mut Ui, models: bool) -> Rect {
    ui.allocate_ui(CREDITS_SIZE, |ui| {
        ui.set_min_size(CREDITS_SIZE);
        if models { crate::credits::models_ui(ui) } else { crate::credits::contributors_ui(ui) }
    })
    .response
    .rect
}

pub fn about(_app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.vertical_centered(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(64.0, 64.0), Sense::hover());
        paint_logo(ui.painter(), r);
        ui.label(egui::RichText::new("The AllCraft Deck").font(theme::bold(22.0)));
        ui.label(egui::RichText::new(version_line()).color(t.text_dim));
        ui.add_space(6.0);
        ui.label("Presentations and slide shows, rebuilt from scratch in pure Rust.");
        ui.label("Part of The AllCraft Studio open creative & office suite.");
        ui.add_space(6.0);
        ui.hyperlink_to("The AllCraft Studio — the-allcraft.pages.dev", deckcraft_engine::links::WEBSITE);
        ui.hyperlink_to("the-allcraft.pages.dev/deck/", deckcraft_engine::links::APP_PAGE);
        ui.hyperlink_to("DeckCraft on GitHub", deckcraft_engine::links::GITHUB);
        ui.add_space(6.0);
        ui.label(egui::RichText::new("MIT OR Apache-2.0 · © 2026 ArtCraft Team and the DeckCraft contributors").size(11.0).color(t.text_faint));
    });
}

pub fn version_line() -> String {
    let mut s = format!("Version {}", env!("CARGO_PKG_VERSION"));
    let sha = option_env!("DECKCRAFT_BUILD_SHA").map(|s| s.get(..9).unwrap_or(s)).filter(|s| !s.is_empty());
    let date = option_env!("DECKCRAFT_BUILD_DATE").filter(|s| !s.is_empty());
    match (sha, date) {
        (Some(sha), Some(date)) => s.push_str(&format!(" ({sha}, {date})")),
        (Some(x), None) | (None, Some(x)) => s.push_str(&format!(" ({x})")),
        (None, None) => {}
    }
    s
}

/// The AllCraft Deck mark, drawn in code: a slide card with a play triangle in warm orange.
pub fn paint_logo(p: &egui::Painter, r: Rect) {
    let orange = Color32::from_rgb(0xF9, 0x73, 0x16);
    let deep = Color32::from_rgb(0xC2, 0x41, 0x0C);
    let card = r.shrink(r.width() * 0.08);
    p.rect_filled(card.translate(vec2(r.width() * 0.04, r.width() * 0.05)), CornerRadius::same((r.width() * 0.14) as u8), deep);
    p.rect_filled(card, CornerRadius::same((r.width() * 0.14) as u8), orange);
    let c = card.center();
    let s = card.width() * 0.22;
    p.add(egui::Shape::convex_polygon(
        vec![pos2(c.x - s * 0.7, c.y - s), pos2(c.x + s, c.y), pos2(c.x - s * 0.7, c.y + s)],
        Color32::WHITE,
        Stroke::NONE,
    ));
}

#[cfg(test)]
mod about_credits_tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::{CREDITS_SIZE, credits_block};
    use crate::dialogs::Dialog;
    use crate::{Services, SlideApp};

    fn tab_of(h: &Harness<'_, SlideApp>) -> String {
        let d = h.state().dialog.as_ref().expect("the About dialog is open");
        d.fields.get("tab").cloned().unwrap_or_else(|| "about".into())
    }

    #[test]
    fn about_dialog_tabs_keep_responding() {
        let mut app = SlideApp::new(deckcraft_engine::Session::with_new(), Services::default());
        app.dialog = Some(Dialog::new("about"));
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_step_dt(1.0 / 60.0).build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        assert_eq!(tab_of(&h), "about");
        for label in ["Contributors", "Models", "About", "Models", "Contributors"] {
            h.get_by_label(label).click();
            h.run_steps(3);
            assert_eq!(tab_of(&h), label.to_lowercase(), "clicking {label} switches the tab");
        }
    }

    #[test]
    fn credits_get_their_own_block_below_the_tab_row() {
        for models in [false, true] {
            let ctx = egui::Context::default();
            let mut tabs = egui::Rect::NOTHING;
            let mut block = egui::Rect::NOTHING;
            let mut whole = egui::Rect::NOTHING;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                tabs = ui.label("About").rect;
                block = credits_block(ui, models);
                whole = ui.min_rect();
            });
            out.textures_delta.clear();
            assert_eq!(block.size(), CREDITS_SIZE, "models={models}");
            assert!(block.min.y >= tabs.max.y, "the credits start below the tab row (models={models})");
            assert!(whole.max.y >= block.max.y, "the dialog grows to hold the block (models={models})");
            assert!(whole.height() > CREDITS_SIZE.y, "the tab row is not squeezed into the credits height (models={models})");
        }
    }
}
