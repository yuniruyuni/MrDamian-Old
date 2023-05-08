use miette::{IntoDiagnostic, Result, WrapErr};

use crate::error::MrDamianError;

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub enum MenuMode {
    Hide,
    Open,
}

impl From<MenuMode> for CustomMenuItem {
    fn from(mode: MenuMode) -> Self {
        use MenuMode::*;
        match mode {
            Hide => CustomMenuItem::new("hide".to_string(), "Hide"),
            Open => CustomMenuItem::new("open".to_string(), "Open"),
        }
    }
}

pub fn menu_from(mode: MenuMode) -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(mode.into())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

fn hide_window(app: &AppHandle) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or(MrDamianError::WindowNotFound)
        .into_diagnostic()?;
    window.hide().into_diagnostic()?;
    app.tray_handle()
        .set_menu(menu_from(MenuMode::Hide))
        .into_diagnostic()
        .context("failed to change hide window system tray menu")
}

fn show_window(app: &AppHandle) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or(MrDamianError::WindowNotFound)?;
    window.show().into_diagnostic()?;
    app.tray_handle()
        .set_menu(menu_from(MenuMode::Open))
        .into_diagnostic()
        .context("failed to change open window system tray menu")
}

fn flip_window_visibility(app: &AppHandle) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or(MrDamianError::WindowNotFound)?;
    if window.is_visible().unwrap_or(false) {
        hide_window(app)
    } else {
        show_window(app)
    }
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    use SystemTrayEvent::*;
    match event {
        DoubleClick { .. } => {
            flip_window_visibility(app).expect("failed to flip main window visibility.")
        }
        MenuItemClick { id, .. } => match id.as_str() {
            "quit" => std::process::exit(0),
            "hide" => hide_window(app).expect("failed to hide main window."),
            "open" => show_window(app).expect("failed to show main window."),
            _ => {}
        },
        _ => {}
    }
}
