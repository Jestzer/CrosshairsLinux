use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box as GtkBox, Button, DrawingArea, Entry, Label, Orientation,
};

use crate::config::Config;

pub fn build_interface(
    app: &gtk4::Application,
    config: Rc<RefCell<Config>>,
    overlay_drawing_area: DrawingArea,
) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Crosshairs Interface")
        .default_width(400)
        .default_height(475)
        .resizable(false)
        .build();

    // Apply dark theme CSS
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        r#"
        .interface-window {
            background-color: #2D2B2B;
        }
        .interface-button {
            background-color: #464646;
            color: white;
            border: 2px solid #1E699B;
            border-radius: 0;
            min-height: 30px;
            min-width: 60px;
        }
        .interface-button:hover {
            background-color: #808080;
        }
        .interface-button:active {
            background-color: #CECECE;
        }
        .coord-entry {
            background-color: #464646;
            color: white;
            min-width: 60px;
        }
        .interface-label {
            color: white;
        }
        "#,
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not get default display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
    );

    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.add_css_class("interface-window");

    // Spacer
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    main_box.append(&spacer);

    // Y coordinate entry (above the arrow cross)
    let y_entry = Entry::new();
    y_entry.add_css_class("coord-entry");
    y_entry.set_text(&format!("{}", config.borrow().y_offset as i64));
    y_entry.set_halign(Align::Center);
    y_entry.set_width_chars(6);
    y_entry.set_max_width_chars(8);
    y_entry.set_margin_bottom(5);

    let y_label = Label::new(Some("Y"));
    y_label.add_css_class("interface-label");
    y_label.set_margin_bottom(2);

    let y_box = GtkBox::new(Orientation::Vertical, 0);
    y_box.set_halign(Align::Center);
    y_box.append(&y_label);
    y_box.append(&y_entry);
    main_box.append(&y_box);

    // Arrow buttons in a cross layout
    let arrows_box = GtkBox::new(Orientation::Vertical, 4);
    arrows_box.set_halign(Align::Center);
    arrows_box.set_margin_top(10);
    arrows_box.set_margin_bottom(10);

    let up_button = Button::with_label("\u{2191}");
    up_button.add_css_class("interface-button");
    up_button.set_halign(Align::Center);

    let middle_row = GtkBox::new(Orientation::Horizontal, 4);
    middle_row.set_halign(Align::Center);

    let left_button = Button::with_label("\u{2190}");
    left_button.add_css_class("interface-button");

    let center_spacer = GtkBox::new(Orientation::Horizontal, 0);
    center_spacer.set_size_request(64, -1);

    let right_button = Button::with_label("\u{2192}");
    right_button.add_css_class("interface-button");

    middle_row.append(&left_button);
    middle_row.append(&center_spacer);
    middle_row.append(&right_button);

    let down_button = Button::with_label("\u{2193}");
    down_button.add_css_class("interface-button");
    down_button.set_halign(Align::Center);

    arrows_box.append(&up_button);
    arrows_box.append(&middle_row);
    arrows_box.append(&down_button);

    main_box.append(&arrows_box);

    // Hint label for keyboard control
    let hint_label = Label::new(Some("Use arrow keys to move (hold to repeat)"));
    hint_label.add_css_class("interface-label");
    hint_label.set_opacity(0.6);
    main_box.append(&hint_label);

    // X coordinate entry (below the arrow cross)
    let x_entry = Entry::new();
    x_entry.add_css_class("coord-entry");
    x_entry.set_text(&format!("{}", config.borrow().x_offset as i64));
    x_entry.set_halign(Align::Center);
    x_entry.set_width_chars(6);
    x_entry.set_max_width_chars(8);
    x_entry.set_margin_top(5);

    let x_label = Label::new(Some("X"));
    x_label.add_css_class("interface-label");
    x_label.set_margin_top(2);

    let x_box = GtkBox::new(Orientation::Vertical, 0);
    x_box.set_halign(Align::Center);
    x_box.append(&x_label);
    x_box.append(&x_entry);
    main_box.append(&x_box);

    // Spacer
    let spacer2 = GtkBox::new(Orientation::Vertical, 0);
    spacer2.set_vexpand(true);
    main_box.append(&spacer2);

    // Bottom bar: Save and Reset buttons
    let bottom_bar = GtkBox::new(Orientation::Horizontal, 8);
    bottom_bar.set_margin_bottom(10);
    bottom_bar.set_margin_start(10);
    bottom_bar.set_margin_end(10);
    bottom_bar.set_halign(Align::End);

    let version_label = Label::new(Some("v1.0.0"));
    version_label.add_css_class("interface-label");
    version_label.set_hexpand(true);
    version_label.set_halign(Align::Start);
    bottom_bar.append(&version_label);

    let reset_button = Button::with_label("Reset Position");
    reset_button.add_css_class("interface-button");

    let save_button = Button::with_label("Save Position");
    save_button.add_css_class("interface-button");

    bottom_bar.append(&reset_button);
    bottom_bar.append(&save_button);

    main_box.append(&bottom_bar);

    window.set_child(Some(&main_box));

    // --- Wire up callbacks ---

    // Helper: update overlay and entries after config change
    let update_ui = {
        let config = config.clone();
        let overlay_da = overlay_drawing_area.clone();
        let x_entry = x_entry.clone();
        let y_entry = y_entry.clone();
        Rc::new(move || {
            let c = config.borrow();
            x_entry.set_text(&format!("{}", c.x_offset as i64));
            y_entry.set_text(&format!("{}", c.y_offset as i64));
            overlay_da.queue_draw();
        })
    };

    // Arrow buttons: single click moves once
    {
        let config = config.clone();
        let update = update_ui.clone();
        up_button.connect_clicked(move |_| {
            config.borrow_mut().y_offset -= 1.0;
            update();
        });
    }
    {
        let config = config.clone();
        let update = update_ui.clone();
        down_button.connect_clicked(move |_| {
            config.borrow_mut().y_offset += 1.0;
            update();
        });
    }
    {
        let config = config.clone();
        let update = update_ui.clone();
        left_button.connect_clicked(move |_| {
            config.borrow_mut().x_offset -= 1.0;
            update();
        });
    }
    {
        let config = config.clone();
        let update = update_ui.clone();
        right_button.connect_clicked(move |_| {
            config.borrow_mut().x_offset += 1.0;
            update();
        });
    }

    // Keyboard arrow keys: the OS handles key repeat natively when held
    let key_controller = gtk4::EventControllerKey::new();
    {
        let config = config.clone();
        let update = update_ui.clone();
        key_controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
            match keyval {
                gdk::Key::Up => {
                    config.borrow_mut().y_offset -= 1.0;
                    update();
                    gtk4::glib::Propagation::Stop
                }
                gdk::Key::Down => {
                    config.borrow_mut().y_offset += 1.0;
                    update();
                    gtk4::glib::Propagation::Stop
                }
                gdk::Key::Left => {
                    config.borrow_mut().x_offset -= 1.0;
                    update();
                    gtk4::glib::Propagation::Stop
                }
                gdk::Key::Right => {
                    config.borrow_mut().x_offset += 1.0;
                    update();
                    gtk4::glib::Propagation::Stop
                }
                _ => gtk4::glib::Propagation::Proceed,
            }
        });
    }
    window.add_controller(key_controller);

    // X entry: update on activate (Enter key)
    {
        let config = config.clone();
        let overlay_da = overlay_drawing_area.clone();
        x_entry.connect_activate(move |entry| {
            if let Ok(val) = entry.text().parse::<f64>() {
                config.borrow_mut().x_offset = val;
                overlay_da.queue_draw();
            }
        });
    }

    // Y entry: update on activate (Enter key)
    {
        let config = config.clone();
        let overlay_da = overlay_drawing_area.clone();
        y_entry.connect_activate(move |entry| {
            if let Ok(val) = entry.text().parse::<f64>() {
                config.borrow_mut().y_offset = val;
                overlay_da.queue_draw();
            }
        });
    }

    // Save
    {
        let config = config.clone();
        save_button.connect_clicked(move |_| {
            config.borrow().save();
        });
    }

    // Reset
    {
        let config = config.clone();
        let update = update_ui.clone();
        reset_button.connect_clicked(move |_| {
            let default = Config::default();
            let mut c = config.borrow_mut();
            c.x_offset = default.x_offset;
            c.y_offset = default.y_offset;
            drop(c);
            update();
        });
    }

    // Auto-save and quit the entire application when the interface window is closed
    {
        let config = config.clone();
        let app = app.clone();
        window.connect_close_request(move |_| {
            config.borrow().save();
            app.quit();
            gtk4::glib::Propagation::Proceed
        });
    }

    window.present();

    window
}
