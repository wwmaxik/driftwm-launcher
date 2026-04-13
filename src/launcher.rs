use crate::app_entry::AppEntry;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gtk4::prelude::*;
use gtk4::{Label, ListBox, ListBoxRow};
use std::process::Command;

pub fn populate_list(list_box: &ListBox, apps: &[AppEntry]) {
    // Clear existing items
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    // Add all apps
    for app in apps {
        let row = create_app_row(app);
        list_box.append(&row);
    }
}

pub fn filter_list(list_box: &ListBox, apps: &[AppEntry], query: &str) {
    // Clear existing items
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    if query.is_empty() {
        // Show all apps
        for app in apps {
            let row = create_app_row(app);
            list_box.append(&row);
        }
        return;
    }

    // Fuzzy search
    let matcher = SkimMatcherV2::default();
    let mut scored_apps: Vec<(i64, &AppEntry)> = apps
        .iter()
        .filter_map(|app| {
            let name_score = matcher.fuzzy_match(&app.name, query).unwrap_or(0);
            let desc_score = matcher.fuzzy_match(&app.description, query).unwrap_or(0);
            let score = name_score.max(desc_score);

            if score > 0 {
                Some((score, app))
            } else {
                None
            }
        })
        .collect();

    // Sort by score (highest first)
    scored_apps.sort_by(|a, b| b.0.cmp(&a.0));

    // Add filtered apps
    for (_, app) in scored_apps {
        let row = create_app_row(app);
        list_box.append(&row);
    }
}

fn create_app_row(app: &AppEntry) -> ListBoxRow {
    let row = ListBoxRow::new();

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);
    hbox.set_margin_start(12);
    hbox.set_margin_end(12);

    // Icon (if available)
    if let Some(icon_name) = &app.icon {
        let icon = gtk4::Image::from_icon_name(icon_name);
        icon.set_pixel_size(32);
        hbox.append(&icon);
    }

    // Text container
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 4);

    let name_label = Label::new(Some(&app.name));
    name_label.set_halign(gtk4::Align::Start);
    name_label.add_css_class("app-name");

    let desc_label = Label::new(Some(&app.description));
    desc_label.set_halign(gtk4::Align::Start);
    desc_label.add_css_class("app-description");
    desc_label.add_css_class("dim-label");
    desc_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    vbox.append(&name_label);
    if !app.description.is_empty() {
        vbox.append(&desc_label);
    }

    hbox.append(&vbox);
    row.set_child(Some(&hbox));

    // Store app data using widget name as workaround
    row.set_widget_name(&format!("{}|{}", app.exec, app.terminal));

    row
}

pub fn launch_selected(list_box: &ListBox) {
    if let Some(row) = list_box.selected_row() {
        let widget_name = row.widget_name();
        let parts: Vec<&str> = widget_name.split('|').collect();
        if parts.len() == 2 {
            let exec = parts[0];
            let terminal = parts[1] == "true";
            launch_app(exec, terminal);
        }
    }
}

fn launch_app(exec: &str, terminal: bool) {
    // Remove field codes (%f, %F, %u, %U, etc.)
    let cleaned_exec = exec
        .split_whitespace()
        .filter(|s| !s.starts_with('%'))
        .collect::<Vec<_>>()
        .join(" ");

    if terminal {
        // Launch in terminal (use common terminal emulators)
        let terminals = ["alacritty", "kitty", "foot", "gnome-terminal", "konsole"];
        for term in terminals {
            if Command::new(term)
                .arg("-e")
                .arg(&cleaned_exec)
                .spawn()
                .is_ok()
            {
                return;
            }
        }
    }

    // Launch directly
    let parts: Vec<&str> = cleaned_exec.split_whitespace().collect();
    if let Some((cmd, args)) = parts.split_first() {
        let _ = Command::new(cmd).args(args).spawn();
    }
}
