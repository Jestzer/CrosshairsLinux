mod config;
mod interface;
mod overlay;

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;

fn main() {
    let app = gtk4::Application::builder()
        .application_id("com.crosshairs.linux")
        .build();

    app.connect_activate(|app| {
        let config = Rc::new(RefCell::new(config::Config::load()));

        let (_overlay_window, overlay_drawing_area) =
            overlay::build_overlay(app, config.clone());

        let _interface_window =
            interface::build_interface(app, config, overlay_drawing_area);
    });

    app.run();
}
