mod app_entry;
mod config;
mod launcher;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider, Entry, ListBox, ScrolledWindow};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const APP_ID: &str = "com.github.driftwm.launcher";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../style.css"));
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Could not connect to display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Load config
    let cfg = config::LauncherConfig::load();

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(cfg.width)
        .default_height(cfg.height)
        .build();

    // Initialize layer shell
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    // Anchor based on config
    match cfg.position {
        config::Position::Top => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Left, false);
            window.set_anchor(Edge::Right, false);
            window.set_anchor(Edge::Bottom, false);
            window.set_margin(Edge::Top, cfg.margin_top);
        }
        config::Position::Center => {
            window.set_anchor(Edge::Top, false);
            window.set_anchor(Edge::Left, false);
            window.set_anchor(Edge::Right, false);
            window.set_anchor(Edge::Bottom, false);
        }
        config::Position::Bottom => {
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, false);
            window.set_anchor(Edge::Right, false);
            window.set_anchor(Edge::Top, false);
            window.set_margin(Edge::Bottom, cfg.margin_top);
        }
    }

    // Main container
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    // Search entry
    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Type to search applications..."));
    search_entry.add_css_class("search-entry");

    // Results list
    let list_box = ListBox::new();
    list_box.add_css_class("results-list");

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&list_box));
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(300);

    vbox.append(&search_entry);
    vbox.append(&scrolled);

    window.set_child(Some(&vbox));

    // Load applications
    let apps = app_entry::load_applications();
    launcher::populate_list(&list_box, &apps);

    // Search functionality
    let list_clone = list_box.clone();
    let list_clone2 = list_box.clone();
    let apps_clone = apps.clone();
    search_entry.connect_changed(move |entry| {
        let query = entry.text().to_string();
        launcher::filter_list(&list_clone, &apps_clone, &query);
    });

    // Handle Enter key
    let window_clone = window.clone();
    search_entry.connect_activate(move |_| {
        // Launch first item in list
        launcher::launch_selected(&list_clone2);
        window_clone.close();
    });

    // Handle Escape key
    let window_clone2 = window.clone();
    let controller = gtk4::EventControllerKey::new();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            window_clone2.close();
            gtk4::glib::Propagation::Stop
        } else {
            gtk4::glib::Propagation::Proceed
        }
    });
    window.add_controller(controller);

    window.present();
    search_entry.grab_focus();
}
