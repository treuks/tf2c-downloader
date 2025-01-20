use std::fs::read;

use eframe::{
    egui::{self, Context, FontData},
    epaint::text::FontFamily,
};

use egui::{Color32, Rounding, Stroke, Style, Theme};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};

fn load_system_font(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    let handle = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap();

    let buf: Vec<u8> = match handle {
        Handle::Memory { bytes, .. } => bytes.to_vec(),
        Handle::Path { path, .. } => read(path).unwrap(),
    };

    const FONT_SYSTEM_SANS_SERIF: &'static str = "System Sans Serif";

    fonts.font_data.insert(
        FONT_SYSTEM_SANS_SERIF.to_owned(),
        FontData::from_owned(buf).into(),
    );

    if let Some(vec) = fonts.families.get_mut(&FontFamily::Proportional) {
        vec.insert(0, FONT_SYSTEM_SANS_SERIF.to_owned());
    }

    if let Some(vec) = fonts.families.get_mut(&FontFamily::Monospace) {
        vec.push(FONT_SYSTEM_SANS_SERIF.to_owned());
    }

    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    ctx.set_fonts(fonts);
}

fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, tf2_theme);
    ctx.style_mut_of(Theme::Dark, tf2_theme);
}

const BG_COLOR: Color32 = Color32::from_rgb(57, 53, 50);
const PRIMARY_TEXT_COLOR: Color32 = Color32::from_rgb(199, 188, 162);
const BUTTON_TEXT_COLOR: Color32 = BG_COLOR;
const BUTTON_COLOR: Color32 = Color32::from_rgb(163, 152, 132);
const ACTIVE_BUTTON_COLOR: Color32 = Color32::from_rgb(189, 183, 164);
const WIDGET_BG_COLOR: Color32 = Color32::from_rgb(39, 36, 34);
const WIDGET_FG_COLOR: Color32 = PRIMARY_TEXT_COLOR;

fn tf2_theme(style: &mut Style) {
    style.visuals.dark_mode = true;
    style.visuals.extreme_bg_color = WIDGET_BG_COLOR;
    style.visuals.panel_fill = BG_COLOR;
    style.visuals.faint_bg_color = Color32::from_rgb(70, 65, 61);

    style.visuals.window_fill = BG_COLOR;
    style.visuals.window_stroke = Stroke {
        width: 1.0,
        color: Color32::from_rgb(74, 69, 61),
    };
    style.visuals.window_rounding = Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: 0.0,
        se: 0.0,
    };

    style.visuals.widgets.inactive.bg_fill = WIDGET_BG_COLOR;
    style.visuals.widgets.inactive.bg_stroke = Stroke {
        width: 1.0,
        color: BUTTON_COLOR,
    };
    style.visuals.widgets.inactive.fg_stroke = Stroke {
        width: 1.0,
        color: BUTTON_TEXT_COLOR,
    };

    style.visuals.widgets.noninteractive.fg_stroke = Stroke {
        width: 1.0,
        color: PRIMARY_TEXT_COLOR,
    };

    style.visuals.widgets.inactive.weak_bg_fill = BUTTON_COLOR;

    style.visuals.widgets.hovered.weak_bg_fill = ACTIVE_BUTTON_COLOR;

    style.visuals.widgets.active.weak_bg_fill = BUTTON_COLOR;
    style.visuals.widgets.hovered.fg_stroke = Stroke {
        width: 0.0,
        color: BUTTON_TEXT_COLOR,
    };
    style.visuals.widgets.active.fg_stroke = Stroke {
        width: 1.0,
        color: BUTTON_TEXT_COLOR,
    };
    style.visuals.widgets.active.bg_stroke = Stroke {
        width: 0.0,
        color: BUTTON_COLOR,
    }
}

pub fn setup_tf2theme(ctx: &egui::Context) {
    setup_custom_style(ctx);
    load_system_font(ctx);
}
