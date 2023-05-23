// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;
mod operation;
mod presentation;
mod repository;
mod usecase;

mod config;

use miette::{IntoDiagnostic, Result, WrapErr};
use std::sync::Mutex;
use tauri::{generate_context, generate_handler, Builder, Manager, SystemTray, WindowEvent};

use presentation::tray;
use repository::Repositories;

fn gen_bindings() {
    use presentation::command::*;
    tauri_specta::ts::export(
        specta::collect_types![
            component::candidates,
            component::create_component,
            editor::editor,
            editor::update_editor,
            edge::add_edge,
            edge::remove_edge,
            edge::set_assignment,
        ],
        "../src/bindings.ts",
    )
    .unwrap();
}

fn main() -> Result<()> {
    use presentation::command::*;

    #[cfg(debug_assertions)]
    gen_bindings();

    let system_tray = SystemTray::new().with_menu(tray::menu_from(tray::MenuMode::Hide));

    Builder::default()
        .invoke_handler(generate_handler![
            component::candidates,
            component::create_component,
            editor::editor,
            editor::update_editor,
            edge::add_edge,
            edge::remove_edge,
            edge::set_assignment,
        ])
        .system_tray(system_tray)
        .on_system_tray_event(tray::on_system_tray_event)
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().expect("failed to hide window");
                api.prevent_close();
            }
        })
        .setup(|app| {
            app.manage(Mutex::new(Repositories::new()));

            Ok(())
        })
        .run(generate_context!())
        .into_diagnostic()
        .context("error while running tauri application")?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn export_bindings() {
        super::gen_bindings();
    }
}
