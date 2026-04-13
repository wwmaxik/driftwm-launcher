use freedesktop_desktop_entry::DesktopEntry;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub description: String,
    pub exec: String,
    pub icon: Option<String>,
    pub terminal: bool,
}

pub fn load_applications() -> Vec<AppEntry> {
    let mut apps = Vec::new();

    // Standard locations for .desktop files
    let home = std::env::var("HOME").unwrap_or_default();
    let local_apps = format!("{}/.local/share/applications", home);

    let search_paths: Vec<&str> = vec![
        "/usr/share/applications",
        "/usr/local/share/applications",
        &local_apps,
    ];

    for path in &search_paths {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension()
                    && ext == "desktop"
                    && let Some(app) = parse_desktop_file(&entry.path())
                {
                    apps.push(app);
                }
            }
        }
    }

    // Sort by name
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn parse_desktop_file(path: &PathBuf) -> Option<AppEntry> {
    let contents = std::fs::read_to_string(path).ok()?;
    let entry = DesktopEntry::decode(path, &contents).ok()?;

    // Skip hidden or NoDisplay entries
    if entry.no_display() {
        return None;
    }

    let name = entry.name(None)?.to_string();
    let description = entry
        .comment(None)
        .unwrap_or(std::borrow::Cow::Borrowed(""))
        .to_string();
    let exec = entry.exec()?.to_string();
    let icon = entry.icon().map(|s| s.to_string());
    let terminal = entry.terminal();

    Some(AppEntry {
        name,
        description,
        exec,
        icon,
        terminal,
    })
}
