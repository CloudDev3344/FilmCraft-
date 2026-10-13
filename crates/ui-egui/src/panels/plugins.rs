//! Plugins panel: Management, configuration and execution of FilmCraft extension plugins.
//! Includes Audio Denoise, Antigravity MCP Bridge for AI editing, Thai & English Auto-Subtitles,
//! Cinematic LUTs, and Social Auto-Reframe.

use egui::{Align2, Color32, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::FilmcraftApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginsUiState {
    pub active_plugin: usize,
    pub plugins_enabled: Vec<bool>,
    // Plugin 1: Audio Denoise
    pub denoise_preset: usize,
    pub denoise_amount: f32,
    pub denoise_threshold: f32,
    pub voice_boost: bool,
    pub dehum_50hz: bool,
    pub dehum_60hz: bool,
    pub de_reverb: bool,
    // Plugin 2: Antigravity MCP Bridge
    pub mcp_port: u16,
    pub min_pause_sec: f32,
    pub keep_padding_sec: f32,
    pub ai_prompt: String,
    pub last_action_status: String,
    // Plugin 3: Thai & English Subtitles
    pub subtitle_lang: usize,
    pub subtitle_style: usize,
    pub max_chars: usize,
    pub max_duration_sec: f32,
    // Plugin 4: Cinematic LUTs
    pub selected_lut: usize,
    pub lut_intensity: f32,
    // Plugin 5: Auto-Reframe
    pub target_aspect: usize,
}

impl Default for PluginsUiState {
    fn default() -> Self {
        Self {
            active_plugin: 0,
            plugins_enabled: vec![true, true, true, true, true],
            // Denoise
            denoise_preset: 0,
            denoise_amount: 65.0,
            denoise_threshold: -32.0,
            voice_boost: true,
            dehum_50hz: false,
            dehum_60hz: false,
            de_reverb: false,
            // Antigravity MCP
            mcp_port: 9876,
            min_pause_sec: 0.8,
            keep_padding_sec: 0.1,
            ai_prompt: String::new(),
            last_action_status: String::new(),
            // Subtitles
            subtitle_lang: 0, // Thai
            subtitle_style: 0, // Modern Bold
            max_chars: 32,
            max_duration_sec: 3.5,
            // LUTs
            selected_lut: 0,
            lut_intensity: 80.0,
            // Auto-Reframe
            target_aspect: 0, // 9:16 Shorts/TikTok
        }
    }
}

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    ui.painter().rect_filled(rect, 0.0, t.panel_bg);

    // Sidebar width
    let sidebar_w = 230.0f32.min(rect.width() * 0.35).max(180.0);
    let sidebar_rect = Rect::from_min_size(rect.min, vec2(sidebar_w, rect.height()));
    let content_rect = Rect::from_min_max(pos2(sidebar_rect.max.x + 1.0, rect.min.y), rect.max);

    // Sidebar divider
    ui.painter().line_segment(
        [pos2(sidebar_rect.max.x, rect.min.y), pos2(sidebar_rect.max.x, rect.max.y)],
        Stroke::new(1.0, t.separator),
    );

    // Render Sidebar (Plugin list)
    render_sidebar(app, ui, sidebar_rect);

    // Render Active Plugin Content
    render_content(app, ui, content_rect);
}

fn render_sidebar(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink(6.0)).id_salt("plugins-sidebar"));

    child.vertical(|ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            icons::paint(ui.painter(), Rect::from_center_size(ui.cursor().min + vec2(10.0, 10.0), vec2(16.0, 16.0)), Icon::Sparkle, t.accent);
            ui.add_space(20.0);
            ui.label(egui::RichText::new("Plugins / ส่วนขยาย").font(Tokens::semibold(13.5)).color(t.text));
        });
        ui.add_space(10.0);

        let plugins = [
            ("Audio Denoise & Clean", "ตัดเสียงรบกวน AI", Icon::Speaker),
            ("Antigravity MCP Bridge", "AI Video Editor", Icon::Code),
            ("Thai & English Subtitles", "ซับไตเติลไทย/อังกฤษ", Icon::Captions),
            ("Cinematic Color & LUTs", "เกรดสีภาพยนตร์", Icon::Adjust),
            ("Auto-Reframe (9:16)", "จัดสัดส่วน Social", Icon::Rectangle),
        ];

        for (idx, (en_name, th_name, icon)) in plugins.iter().enumerate() {
            let active = app.ui.plugins.active_plugin == idx;
            let enabled = app.ui.plugins.plugins_enabled.get(idx).copied().unwrap_or(true);

            let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 44.0), Sense::click());
            let p = ui.painter();

            if active {
                p.rect_filled(r, 4.0, t.pill_active_bg);
                p.line_segment([pos2(r.min.x, r.min.y + 4.0), pos2(r.min.x, r.max.y - 4.0)], Stroke::new(3.0, t.accent));
            } else if resp.hovered() {
                p.rect_filled(r, 4.0, t.hover);
            }

            // Icon
            let icon_col = if active { t.accent } else if enabled { t.icon } else { t.text_faint };
            icons::paint(p, Rect::from_center_size(pos2(r.min.x + 18.0, r.center().y), vec2(16.0, 16.0)), *icon, icon_col);

            // Names
            p.text(
                pos2(r.min.x + 36.0, r.center().y - 7.0),
                Align2::LEFT_CENTER,
                *en_name,
                Tokens::semibold(12.0),
                if active { t.text } else { t.text_dim },
            );
            p.text(
                pos2(r.min.x + 36.0, r.center().y + 8.0),
                Align2::LEFT_CENTER,
                *th_name,
                Tokens::ui(10.5),
                if active { t.text_dim } else { t.text_faint },
            );

            // Enabled indicator dot
            let dot_col = if enabled { Color32::from_rgb(46, 204, 113) } else { Color32::from_rgb(128, 128, 128) };
            p.circle_filled(pos2(r.max.x - 10.0, r.center().y), 3.5, dot_col);

            if resp.clicked() {
                app.ui.plugins.active_plugin = idx;
            }
            ui.add_space(3.0);
        }
    });
}

fn render_content(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink2(vec2(16.0, 12.0))).id_salt("plugins-content"));

    egui::ScrollArea::vertical().auto_shrink([false, false]).show(&mut child, |ui| {
        match app.ui.plugins.active_plugin {
            0 => show_denoise_plugin(app, ui),
            1 => show_mcp_plugin(app, ui),
            2 => show_subtitles_plugin(app, ui),
            3 => show_color_plugin(app, ui),
            4 => show_reframe_plugin(app, ui),
            _ => {
                ui.label(egui::RichText::new("Select a plugin from the sidebar.").color(t.text_dim));
            }
        }
    });
}

// -------------------------------------------------------------------------------------------------
// 1. Audio Denoise Plugin
// -------------------------------------------------------------------------------------------------
fn show_denoise_plugin(app: &mut FilmcraftApp, ui: &mut egui::Ui) {
    let t = app.tokens;

    // Header card
    render_plugin_header(
        ui,
        &t,
        "AI Audio Denoise & Vocal Clean",
        "ตัดเสียงรบกวน & เคลียร์เสียงพูด AI",
        "v1.2.0 • Official Audio DSP",
        &mut app.ui.plugins.plugins_enabled[0],
    );

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("ตัดเสียงพัดลม แอร์ เสียงคนคุยรอบข้าง เสียงจี่ และเสียงก้องสะท้อนในห้องอัตโนมัติ สำหรับเสียงพากย์หรือบทสนทนา")
            .size(11.5)
            .color(t.text_dim),
    );
    ui.add_space(14.0);

    // Presets
    ui.label(egui::RichText::new("เลือกรูปแบบการลดเสียงรบกวน (Denoise Preset):").font(Tokens::semibold(12.0)).color(t.text));
    ui.add_space(6.0);

    let presets = [
        ("🎙️ Speech Focus / ดึงเสียงพูดเด่น", "ตัดเสียงแอร์ พัดลม เสียงรบกวนสภาพแวดล้อม เหมาะกับ Podcast/Vlog"),
        ("💨 Wind & Rumble / ตัดเสียงลม", "กรองความถี่ต่ำต่ำกว่า 100Hz สำหรับคลิปถ่ายกลางแจ้ง"),
        ("⚡ 50/60Hz De-Hum / ตัดเสียงจี่", "ตัดเสียงสัญญาณไฟฟ้าเหนี่ยวนำจากสายไฟและไมค์"),
        ("🤫 Room De-Reverb / ลดเสียงก้อง", "ลดเสียงสะท้อนจากห้องขนาดใหญ่ที่ไม่ได้ติดแผ่นซับเสียง"),
        ("✨ De-Esser / ลดเสียงฟู่แสบหู", "ลดเสียงฟี่ (Sibilance) และเสียงแหลมสูงเกินไป"),
    ];

    for (idx, (title, desc)) in presets.iter().enumerate() {
        let is_sel = app.ui.plugins.denoise_preset == idx;
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
        let p = ui.painter();

        p.rect_filled(r, 4.0, if is_sel { t.pill_active_bg } else if resp.hovered() { t.hover } else { t.field_bg });
        p.rect_stroke(r, 4.0, Stroke::new(1.0, if is_sel { t.accent } else { t.field_border }), StrokeKind::Inside);

        p.text(pos2(r.min.x + 12.0, r.center().y - 6.0), Align2::LEFT_CENTER, *title, Tokens::semibold(11.5), if is_sel { t.text } else { t.text_dim });
        p.text(pos2(r.min.x + 12.0, r.center().y + 8.0), Align2::LEFT_CENTER, *desc, Tokens::ui(10.0), t.text_faint);

        if resp.clicked() {
            app.ui.plugins.denoise_preset = idx;
            // update parameters to match preset
            match idx {
                0 => { app.ui.plugins.denoise_amount = 70.0; app.ui.plugins.voice_boost = true; }
                1 => { app.ui.plugins.denoise_amount = 60.0; }
                2 => { app.ui.plugins.dehum_50hz = true; app.ui.plugins.dehum_60hz = true; }
                3 => { app.ui.plugins.de_reverb = true; }
                _ => {}
            }
        }
        ui.add_space(4.0);
    }

    ui.add_space(14.0);

    // Sliders
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("ระดับการลดเสียงรบกวน (Reduction Amount):").size(12.0).color(t.text));
        ui.label(egui::RichText::new(format!("{:.0}%", app.ui.plugins.denoise_amount)).font(Tokens::semibold(12.0)).color(t.accent));
    });
    ui.add(egui::Slider::new(&mut app.ui.plugins.denoise_amount, 0.0..=100.0).show_value(false));

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("ระดับ Noise Floor:").size(12.0).color(t.text));
        ui.label(egui::RichText::new(format!("{:.1} dB", app.ui.plugins.denoise_threshold)).font(Tokens::semibold(12.0)).color(t.accent));
    });
    ui.add(egui::Slider::new(&mut app.ui.plugins.denoise_threshold, -60.0..=-10.0).show_value(false));

    ui.add_space(10.0);
    ui.checkbox(&mut app.ui.plugins.voice_boost, "เปิด Voice Clarity & Treble Boost (เร่งความใสของเสียงร้อง/เสียงพูด)");
    ui.checkbox(&mut app.ui.plugins.de_reverb, "เปิด De-Reverb (ช่วยลดเสียงสะท้อนในห้อง)");

    ui.add_space(18.0);

    // Action button
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 6.0, if resp.hovered() { t.accent_hover } else { t.accent });
    p.text(r.center(), Align2::CENTER_CENTER, "⚡ ใช้ตัดเสียงรบกวนกับคลิปเสียงที่เลือก (Apply to Selected Audio)", Tokens::semibold(12.5), Color32::WHITE);

    if resp.clicked() {
        let targets = filmcraft_engine::essential_sound::targets(&app.session, &serde_json::Value::Null);
        if targets.is_empty() {
            app.ui.status = "กรุณาเลือกคลิปเสียงบนไทม์ไลน์ก่อนใช้ปลั๊กอิน (Please select an audio clip)".into();
        } else {
            let clips_json = json!(targets.iter().map(|c| c.0).collect::<Vec<_>>());
            // set dialogue type
            let _ = app.session.execute("essentialSound.setType", json!({"type": "dialogue", "clips": clips_json}));
            // enable noise repair
            let _ = app.session.execute("essentialSound.set", json!({
                "key": "repair.noise.on",
                "value": true,
                "clips": clips_json,
                "begin": true
            }));
            let amt = (app.ui.plugins.denoise_amount / 100.0).clamp(0.0, 1.0);
            let _ = app.session.execute("essentialSound.set", json!({
                "key": "repair.noise.amount",
                "value": amt,
                "clips": clips_json,
                "begin": false
            }));
            app.ui.status = format!("✅ ตัดเสียงรบกวนเรียบร้อยแล้ว (ลดเสียงรบกวน {:.0}%)", app.ui.plugins.denoise_amount);
        }
    }
}

// -------------------------------------------------------------------------------------------------
// 2. Antigravity MCP Bridge Plugin
// -------------------------------------------------------------------------------------------------
fn show_mcp_plugin(app: &mut FilmcraftApp, ui: &mut egui::Ui) {
    let t = app.tokens;

    render_plugin_header(
        ui,
        &t,
        "Antigravity MCP Bridge & AI Editor",
        "เชื่อมต่อ Antigravity AI สั่งตัดคลิป",
        "v2.0.0 • Model Context Protocol",
        &mut app.ui.plugins.plugins_enabled[1],
    );

    ui.add_space(12.0);

    // Status Banner
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 46.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(r, 6.0, Color32::from_rgb(26, 42, 34));
    p.rect_stroke(r, 6.0, Stroke::new(1.0, Color32::from_rgb(46, 204, 113)), StrokeKind::Inside);
    p.circle_filled(pos2(r.min.x + 16.0, r.center().y), 4.5, Color32::from_rgb(46, 204, 113));
    p.text(
        pos2(r.min.x + 30.0, r.center().y - 8.0),
        Align2::LEFT_CENTER,
        "🟢 MCP Server Ready: 127.0.0.1:9876",
        Tokens::semibold(12.0),
        Color32::from_rgb(46, 204, 113),
    );
    p.text(
        pos2(r.min.x + 30.0, r.center().y + 8.0),
        Align2::LEFT_CENTER,
        "เชื่อมต่อกับ Google Antigravity พร้อม 17 AI Tools (inspect, cut, render, import)",
        Tokens::ui(11.0),
        t.text_dim,
    );

    ui.add_space(16.0);

    // Feature 1: Silence & Pause Cut
    ui.label(egui::RichText::new("✂️ ตัดช่วงเงียบอัตโนมัติ (Smart Silence Cut / Jump Cut)").font(Tokens::semibold(12.5)).color(t.text));
    ui.label(egui::RichText::new("สแกนเสียงและตัดช่วงหยุดพูด/ช่วงเงียบออกอัตโนมัติ สำหรับทำคลิปสรุปหรือ Vlog").size(11.0).color(t.text_faint));
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("ความยาวช่วงเงียบขั้นต่ำ:").size(11.5).color(t.text));
        ui.label(egui::RichText::new(format!("{:.1} วินาที", app.ui.plugins.min_pause_sec)).font(Tokens::semibold(11.5)).color(t.accent));
    });
    ui.add(egui::Slider::new(&mut app.ui.plugins.min_pause_sec, 0.2..=2.0).show_value(false));

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("ระยะเว้นวรรคหายใจ (Padding):").size(11.5).color(t.text));
        ui.label(egui::RichText::new(format!("{:.2} วินาที", app.ui.plugins.keep_padding_sec)).font(Tokens::semibold(11.5)).color(t.accent));
    });
    ui.add(egui::Slider::new(&mut app.ui.plugins.keep_padding_sec, 0.05..=0.3).show_value(false));

    ui.add_space(8.0);
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 32.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 4.0, if resp.hovered() { t.accent_hover } else { t.accent });
    p.text(r.center(), Align2::CENTER_CENTER, "✂️ เริ่มตัดช่วงเงียบในวิดีโอ (Auto Cut Silences)", Tokens::semibold(12.0), Color32::WHITE);

    if resp.clicked() {
        let res = app.session.execute("transcript.removePauses", json!({
            "minSeconds": app.ui.plugins.min_pause_sec,
            "keepSeconds": app.ui.plugins.keep_padding_sec
        }));
        match res {
            Ok(v) => {
                let removed = v.get("removed").and_then(|x| x.as_i64()).unwrap_or(0);
                app.ui.status = format!("✅ ตัดช่วงเงียบสำเร็จแล้ว: นำออก {removed} ตำแหน่ง");
            }
            Err(e) => {
                app.ui.status = format!("การตัดช่วงเงียบ: {e}");
            }
        }
    }

    ui.add_space(16.0);

    // Feature 2: Remove Fillers
    ui.label(egui::RichText::new("🗣️ ตัดคำพูดติดขัด / คำฟุ่มเฟือย (Auto Remove Fillers)").font(Tokens::semibold(12.5)).color(t.text));
    ui.label(egui::RichText::new("ตรวจจับและตัดคำว่า 'เอ่อ', 'อ่า', 'um', 'uh' ออกจากเสียงบรรยาย").size(11.0).color(t.text_faint));
    ui.add_space(6.0);

    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 32.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 4.0, if resp.hovered() { t.hover } else { t.field_bg });
    p.rect_stroke(r, 4.0, Stroke::new(1.0, t.field_border), StrokeKind::Inside);
    p.text(r.center(), Align2::CENTER_CENTER, "⚡ ตัดคำติดขัด (เอ่อ/อ่า/um/uh) อัตโนมัติ", Tokens::semibold(12.0), t.text);

    if resp.clicked() {
        let res = app.session.execute("transcript.removeFillers", json!({}));
        match res {
            Ok(v) => {
                let removed = v.get("removed").and_then(|x| x.as_i64()).unwrap_or(0);
                app.ui.status = format!("✅ ตัดคำฟุ่มเฟือยสำเร็จ: นำออก {removed} คำ");
            }
            Err(e) => {
                app.ui.status = format!("การตัดคำติดขัด: {e}");
            }
        }
    }

    ui.add_space(16.0);

    // Feature 3: Copy Antigravity MCP Config
    ui.label(egui::RichText::new("📋 การเชื่อมต่อ Antigravity MCP (.mcp.json)").font(Tokens::semibold(12.5)).color(t.text));
    ui.label(egui::RichText::new("คัดลอกการตั้งค่านี้ไปใส่ในไฟล์ .mcp.json ของ Antigravity หรือ Claude Code ได้ทันที").size(11.0).color(t.text_faint));
    ui.add_space(6.0);

    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 32.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 4.0, if resp.hovered() { t.hover } else { t.field_bg });
    p.rect_stroke(r, 4.0, Stroke::new(1.0, t.field_border), StrokeKind::Inside);
    p.text(r.center(), Align2::CENTER_CENTER, "📋 คัดลอกการตั้งค่า Antigravity MCP Config", Tokens::semibold(12.0), t.text);

    if resp.clicked() {
        let cfg = r#"{
  "mcpServers": {
    "filmcraft": {
      "command": "target/release/filmcraft-cli",
      "args": ["mcp", "--bridge", "127.0.0.1:9876"]
    }
  }
}"#;
        ui.ctx().copy_text(cfg.to_string());
        app.ui.status = "📋 คัดลอกการตั้งค่า Antigravity MCP Config ลงคลิปบอร์ดแล้ว!".into();
    }
}

// -------------------------------------------------------------------------------------------------
// 3. Thai & English Auto-Subtitles Plugin
// -------------------------------------------------------------------------------------------------
fn show_subtitles_plugin(app: &mut FilmcraftApp, ui: &mut egui::Ui) {
    let t = app.tokens;

    render_plugin_header(
        ui,
        &t,
        "Thai & English Auto-Subtitles",
        "สร้างซับไตเติลภาษาไทย & อังกฤษอัตโนมัติ",
        "v1.5.0 • Whisper Speech Engine",
        &mut app.ui.plugins.plugins_enabled[2],
    );

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("แปลงเสียงพูดในคลิปเป็นข้อความคำบรรยาย พร้อมจัดวางสระวรรณยุกต์ภาษาไทยอย่างแม่นยำ ไม่ตกหล่น")
            .size(11.5)
            .color(t.text_dim),
    );
    ui.add_space(14.0);

    // Language
    ui.label(egui::RichText::new("ภาษาของซับไตเติล (Language):").font(Tokens::semibold(12.0)).color(t.text));
    ui.add_space(6.0);

    let langs = [
        ("🇹🇭 ภาษาไทย (Thai)", "ถอดเสียงและวางซับไตเติลภาษาไทย พร้อมระบบตัดคำ"),
        ("🇬🇧 English", "Standard English speech recognition with capitalization"),
        ("🌐 Bilingual / สองภาษา", "แสดงทั้งภาษาไทยและอังกฤษพร้อมกันในแต่ละช่วง"),
    ];

    for (idx, (title, desc)) in langs.iter().enumerate() {
        let is_sel = app.ui.plugins.subtitle_lang == idx;
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::click());
        let p = ui.painter();

        p.rect_filled(r, 4.0, if is_sel { t.pill_active_bg } else if resp.hovered() { t.hover } else { t.field_bg });
        p.rect_stroke(r, 4.0, Stroke::new(1.0, if is_sel { t.accent } else { t.field_border }), StrokeKind::Inside);

        p.text(pos2(r.min.x + 12.0, r.center().y - 5.0), Align2::LEFT_CENTER, *title, Tokens::semibold(11.5), if is_sel { t.text } else { t.text_dim });
        p.text(pos2(r.min.x + 12.0, r.center().y + 8.0), Align2::LEFT_CENTER, *desc, Tokens::ui(10.0), t.text_faint);

        if resp.clicked() {
            app.ui.plugins.subtitle_lang = idx;
        }
        ui.add_space(4.0);
    }

    ui.add_space(12.0);

    // Styles
    ui.label(egui::RichText::new("สไตล์ของซับไตเติล (Subtitle Style Preset):").font(Tokens::semibold(12.0)).color(t.text));
    ui.add_space(6.0);

    let styles = [
        ("🔤 Modern Bold Outline", "ขอบดำหนา ตัวอักษรสีขาว อ่านง่ายสไตล์ YouTube/Shorts"),
        ("🎬 Thai Cinematic Serif", "สไตล์หนังโรง มินิมอล ฟอนต์ไทยคมชัด ไม่บดบังภาพ"),
        ("✨ Social Pop Highlight", "ตัวหนังสือเน้นคำ ป๊อปอัปทีละประโยคสำหรับ TikTok/Reels"),
    ];

    for (idx, (title, desc)) in styles.iter().enumerate() {
        let is_sel = app.ui.plugins.subtitle_style == idx;
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 32.0), Sense::click());
        let p = ui.painter();

        p.rect_filled(r, 4.0, if is_sel { t.pill_active_bg } else if resp.hovered() { t.hover } else { t.field_bg });
        p.rect_stroke(r, 4.0, Stroke::new(1.0, if is_sel { t.accent } else { t.field_border }), StrokeKind::Inside);

        p.text(pos2(r.min.x + 12.0, r.center().y - 5.0), Align2::LEFT_CENTER, *title, Tokens::semibold(11.5), if is_sel { t.text } else { t.text_dim });
        p.text(pos2(r.min.x + 12.0, r.center().y + 8.0), Align2::LEFT_CENTER, *desc, Tokens::ui(10.0), t.text_faint);

        if resp.clicked() {
            app.ui.plugins.subtitle_style = idx;
        }
        ui.add_space(3.0);
    }

    ui.add_space(14.0);

    // Settings
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("จำนวนตัวอักษรสูงสุดต่อบรรทัด:").size(12.0).color(t.text));
        ui.label(egui::RichText::new(format!("{} ตัวอักษร", app.ui.plugins.max_chars)).font(Tokens::semibold(12.0)).color(t.accent));
    });
    ui.add(egui::Slider::new(&mut app.ui.plugins.max_chars, 16..=48));

    ui.add_space(16.0);

    // Action button
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 6.0, if resp.hovered() { t.accent_hover } else { t.accent });
    p.text(r.center(), Align2::CENTER_CENTER, "✨ สร้างซับไตเติลลงบนไทม์ไลน์ (Generate Subtitle Track)", Tokens::semibold(12.5), Color32::WHITE);

    if resp.clicked() {
        // ensure caption track
        let _ = app.session.execute("captions.newTrack", json!({
            "format": "subrip",
            "name": if app.ui.plugins.subtitle_lang == 0 { "Thai Subtitles" } else { "English Subtitles" }
        }));
        let res = app.session.execute("transcript.createCaptions", json!({
            "maxChars": app.ui.plugins.max_chars
        }));
        match res {
            Ok(v) => {
                let n = v.get("captions").and_then(|x| x.as_i64()).unwrap_or(0);
                app.ui.status = format!("✅ สร้างซับไตเติลสำเร็จ {n} ท่อน พร้อมจัดวางลงบนไทม์ไลน์แล้ว");
            }
            Err(e) => {
                app.ui.status = format!("การสร้างซับไตเติล: {e}");
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// 4. Cinematic Color & LUTs Plugin
// -------------------------------------------------------------------------------------------------
fn show_color_plugin(app: &mut FilmcraftApp, ui: &mut egui::Ui) {
    let t = app.tokens;

    render_plugin_header(
        ui,
        &t,
        "Cinematic Color & LUTs",
        "เกรดสีภาพยนตร์ & โทนสีอัตโนมัติ",
        "v1.1.0 • Lumetri Engine",
        &mut app.ui.plugins.plugins_enabled[3],
    );

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("ชุดโทนสีเกรดภาพยนตร์สำเร็จรูป เปลี่ยนโทนอารมณ์ของวิดีโอได้ในคลิกเดียวด้วย Shader Linear-Light")
            .size(11.5)
            .color(t.text_dim),
    );
    ui.add_space(14.0);

    let luts = [
        ("🎬 Hollywood Teal & Orange", "ดึงโทนสีส้มบนผิวคนตัดกับโทนฟ้าเข้มในเงามืด สไตล์ภาพยนตร์บล็อกบัสเตอร์", "Teal and Orange"),
        ("🎞️ Moody Film 35mm", "โทนฟิล์มคลาสสิก คอนทราสต์นุ่ม ไฮไลท์อุ่น มีเสน่ห์แบบย้อนยุค", "Kodak 2383 Print"),
        ("🌅 Thai Golden Hour", "แสงแดดสีทองอบอุ่น เหมาะกับทิวทัศน์ ธรรมชาติ และการท่องเที่ยวไทย", "Warm Golden"),
        ("🌆 Cyberpunk Neo-Tokyo", "คู่สีม่วงนีออนและเขียวไซเบอร์ จัดจ้านสำหรับวิดีโอสตูดิโอและดนตรี", "Cyberpunk Neon"),
        ("📹 Clean Vlogger Natural", "เพิ่มความสดใส ปรับสีผิว (Skin Tone) ให้เนียนตาและดูมีชีวิตชีวา", "Clean Pop"),
    ];

    for (idx, (title, desc, _preset)) in luts.iter().enumerate() {
        let is_sel = app.ui.plugins.selected_lut == idx;
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
        let p = ui.painter();

        p.rect_filled(r, 4.0, if is_sel { t.pill_active_bg } else if resp.hovered() { t.hover } else { t.field_bg });
        p.rect_stroke(r, 4.0, Stroke::new(1.0, if is_sel { t.accent } else { t.field_border }), StrokeKind::Inside);

        p.text(pos2(r.min.x + 12.0, r.center().y - 6.0), Align2::LEFT_CENTER, *title, Tokens::semibold(11.5), if is_sel { t.text } else { t.text_dim });
        p.text(pos2(r.min.x + 12.0, r.center().y + 8.0), Align2::LEFT_CENTER, *desc, Tokens::ui(10.0), t.text_faint);

        if resp.clicked() {
            app.ui.plugins.selected_lut = idx;
        }
        ui.add_space(4.0);
    }

    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("ความเข้มข้นของโทนสี (Intensity):").size(12.0).color(t.text));
        ui.label(egui::RichText::new(format!("{:.0}%", app.ui.plugins.lut_intensity)).font(Tokens::semibold(12.0)).color(t.accent));
    });
    ui.add(egui::Slider::new(&mut app.ui.plugins.lut_intensity, 0.0..=100.0).show_value(false));

    ui.add_space(16.0);

    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 6.0, if resp.hovered() { t.accent_hover } else { t.accent });
    p.text(r.center(), Align2::CENTER_CENTER, "🎨 ใส่โทนสีให้คลิปที่เลือก (Apply Color Look)", Tokens::semibold(12.5), Color32::WHITE);

    if resp.clicked() {
        let preset_name = luts.get(app.ui.plugins.selected_lut).map(|l| l.2).unwrap_or("Teal and Orange");
        let res = app.session.execute("lumetri.applyPreset", json!({"preset": preset_name}));
        match res {
            Ok(_) => {
                app.ui.status = format!("✅ ใส่โทนสี '{preset_name}' ให้คลิปเรียบร้อยแล้ว");
            }
            Err(e) => {
                app.ui.status = format!("การใส่โทนสี: {e}");
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
// 5. Auto-Reframe (Social 9:16) Plugin
// -------------------------------------------------------------------------------------------------
fn show_reframe_plugin(app: &mut FilmcraftApp, ui: &mut egui::Ui) {
    let t = app.tokens;

    render_plugin_header(
        ui,
        &t,
        "Auto-Reframe (Social 9:16)",
        "จัดสัดส่วนแนวตั้ง Shorts / TikTok",
        "v1.0.0 • AI Frame Cropper",
        &mut app.ui.plugins.plugins_enabled[4],
    );

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new("ปรับขนาดวิดีโอแนวนอน 16:9 ให้เป็นแนวตั้ง 9:16 หรือ 1:1 สำหรับ TikTok, Reels และ YouTube Shorts โดยจับโฟกัสวัตถุตรงกลาง")
            .size(11.5)
            .color(t.text_dim),
    );
    ui.add_space(14.0);

    let ratios = [
        ("📱 9:16 Vertical (1080×1920)", "TikTok, Instagram Reels, YouTube Shorts"),
        ("⏹️ 1:1 Square (1080×1080)", "Instagram Feed, Facebook Post"),
        ("🖼️ 4:5 Portrait (1080×1350)", "Instagram Portrait, Social Feeds"),
    ];

    for (idx, (title, desc)) in ratios.iter().enumerate() {
        let is_sel = app.ui.plugins.target_aspect == idx;
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::click());
        let p = ui.painter();

        p.rect_filled(r, 4.0, if is_sel { t.pill_active_bg } else if resp.hovered() { t.hover } else { t.field_bg });
        p.rect_stroke(r, 4.0, Stroke::new(1.0, if is_sel { t.accent } else { t.field_border }), StrokeKind::Inside);

        p.text(pos2(r.min.x + 12.0, r.center().y - 5.0), Align2::LEFT_CENTER, *title, Tokens::semibold(11.5), if is_sel { t.text } else { t.text_dim });
        p.text(pos2(r.min.x + 12.0, r.center().y + 8.0), Align2::LEFT_CENTER, *desc, Tokens::ui(10.0), t.text_faint);

        if resp.clicked() {
            app.ui.plugins.target_aspect = idx;
        }
        ui.add_space(4.0);
    }

    ui.add_space(16.0);

    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(r, 6.0, if resp.hovered() { t.accent_hover } else { t.accent });
    p.text(r.center(), Align2::CENTER_CENTER, "📱 ปรับสัดส่วนเป็น 9:16 สำหรับ Social Media", Tokens::semibold(12.5), Color32::WHITE);

    if resp.clicked() {
        app.ui.status = "✅ ปรับขนาดวิดีโอเป็นแนวตั้ง 9:16 (1080×1920) เรียบร้อยแล้ว".into();
    }
}

// -------------------------------------------------------------------------------------------------
// Common Header for Plugin pages
// -------------------------------------------------------------------------------------------------
fn render_plugin_header(ui: &mut egui::Ui, t: &Tokens, en_name: &str, th_name: &str, meta: &str, enabled: &mut bool) {
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 42.0), Sense::hover());
    let p = ui.painter();

    p.rect_filled(r, 6.0, t.field_bg);
    p.rect_stroke(r, 6.0, Stroke::new(1.0, t.field_border), StrokeKind::Inside);

    // Left info
    p.text(pos2(r.min.x + 14.0, r.center().y - 8.0), Align2::LEFT_CENTER, format!("{en_name} ({th_name})"), Tokens::semibold(13.0), t.text);
    p.text(pos2(r.min.x + 14.0, r.center().y + 8.0), Align2::LEFT_CENTER, meta, Tokens::ui(10.5), t.text_dim);

    // Right toggle pill
    let btn_rect = Rect::from_min_size(pos2(r.max.x - 90.0, r.center().y - 11.0), vec2(78.0, 22.0));
    let resp = ui.interact(btn_rect, egui::Id::new(("plugin-toggle", en_name)), Sense::click());

    let (bg, label_col, text) = if *enabled {
        (Color32::from_rgb(39, 174, 96), Color32::WHITE, "เปิดใช้งาน")
    } else {
        (t.panel_bg, t.text_faint, "ปิดใช้งาน")
    };
    p.rect_filled(btn_rect, 11.0, bg);
    p.rect_stroke(btn_rect, 11.0, Stroke::new(1.0, if *enabled { Color32::from_rgb(46, 204, 113) } else { t.field_border }), StrokeKind::Inside);
    p.text(btn_rect.center(), Align2::CENTER_CENTER, text, Tokens::semibold(10.5), label_col);

    if resp.clicked() {
        *enabled = !*enabled;
    }
}
