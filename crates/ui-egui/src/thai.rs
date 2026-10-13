//! Thai fallback fonts for interface and document text, independent of UI language.
//! Looks up installed Thai faces on native systems and installs them into egui FontDefinitions.

use std::sync::{Arc, OnceLock};

use egui::{FontData, FontDefinitions};

const SAMPLE: &str = "ภาษาไทยภาพยนตร์แก้ไขตัดต่อ";
const PREFERRED: &[&str] = &[
    "Leelawadee UI",
    "Leelawadee",
    "Tahoma",
    "Thonburi",
    "Sukhumvit Set",
    "Sathu",
    "Noto Sans Thai UI",
    "Noto Sans Thai",
    "Sarabun",
    "Kanit",
    "Prompt",
    "Cordia New",
    "Angsana New",
    "Browallia New",
    "Segoe UI",
];

pub fn system_font() -> Option<Arc<FontData>> {
    static FONT: OnceLock<Option<Arc<FontData>>> = OnceLock::new();
    FONT.get_or_init(|| find_system_font(SAMPLE, PREFERRED)).clone()
}

fn find_system_font(sample: &str, preferred: &[&str]) -> Option<Arc<FontData>> {
    if cfg!(target_arch = "wasm32") {
        return None;
    }
    filmcraft_text::fonts::scan_system();
    let faces: Vec<_> = filmcraft_text::fonts::all_faces().into_iter().filter(|f| f.info.origin == "system" && !f.info.italic).collect();
    let covers = |f: &filmcraft_text::fonts::Face| f.covers_text(sample);
    let by_weight = |f: &&Arc<filmcraft_text::fonts::Face>| f.info.weight.abs_diff(400);
    let found = preferred.iter().find_map(|name| faces.iter().filter(|f| f.info.family.eq_ignore_ascii_case(name) && covers(f)).min_by_key(by_weight));
    let face = found.or_else(|| faces.iter().filter(|f| f.info.supports_thai && covers(f)).min_by_key(by_weight))?;
    Some(Arc::new(FontData { font: std::borrow::Cow::Owned(face.data()?), index: face.info.index, tweak: Default::default() }))
}

/// Whether a Thai font is available on this system.
pub fn available() -> bool {
    system_font().is_some()
}

/// Append the Thai fallback font to egui's font definitions across all families.
pub fn install(fonts: &mut FontDefinitions) {
    if let Some(font) = system_font() {
        fonts.font_data.insert("system-thai".into(), font);
        for stack in fonts.families.values_mut() {
            stack.push("system-thai".into());
        }
    }
}
