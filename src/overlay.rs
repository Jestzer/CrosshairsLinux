use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use gtk4::cairo;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, DrawingArea};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::config::Config;

pub fn build_overlay(
    app: &gtk4::Application,
    config: Rc<RefCell<Config>>,
) -> (ApplicationWindow, DrawingArea) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Crosshairs Overlay")
        .build();

    // Initialize layer shell before the window is realized
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::None);
    window.set_exclusive_zone(-1);

    // Anchor to all edges for full-screen coverage
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    // Make the window background transparent via CSS
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("window, .background { background: transparent; }");
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not get default display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_opacity(0.9);

    // Drawing area for the crosshair
    let drawing_area = DrawingArea::new();
    drawing_area.set_hexpand(true);
    drawing_area.set_vexpand(true);

    let config_clone = config.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        draw_crosshair(cr, width, height, &config_clone.borrow());
    });

    window.set_child(Some(&drawing_area));

    // After the window is realized, set empty input region for click-through
    window.connect_realize(|win| {
        if let Some(surface) = win.surface() {
            let empty_region = cairo::Region::create();
            surface.set_input_region(&empty_region);
        }
    });

    window.present();

    (window, drawing_area)
}

fn draw_crosshair(cr: &cairo::Context, width: i32, height: i32, config: &Config) {
    // Clear to fully transparent
    cr.set_operator(cairo::Operator::Source);
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
    let _ = cr.paint();

    // Switch back to normal compositing
    cr.set_operator(cairo::Operator::Over);

    // Both offsets are relative to the center of the screen
    let cx = (width as f64 / 2.0) + config.x_offset;
    let cy = (height as f64 / 2.0) + config.y_offset;

    let font_size = 30.0;

    cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    cr.set_font_size(font_size);

    let text = "+";
    let extents = cr.text_extents(text).unwrap();
    // Center the text glyph on (cx, cy)
    let text_x = cx - extents.width() / 2.0 - extents.x_bearing();
    let text_y = cy - extents.height() / 2.0 - extents.y_bearing();

    // Draw white glow: render the text multiple times at offsets with white color and alpha
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.15);
    let glow_steps = 12;
    for radius in [1.0, 2.0, 3.0, 4.0, 5.0] {
        for i in 0..glow_steps {
            let angle = 2.0 * PI * (i as f64) / (glow_steps as f64);
            let dx = radius * angle.cos();
            let dy = radius * angle.sin();
            cr.move_to(text_x + dx, text_y + dy);
            let _ = cr.show_text(text);
        }
    }

    // Draw the crisp black "+" on top
    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.move_to(text_x, text_y);
    let _ = cr.show_text(text);
}
