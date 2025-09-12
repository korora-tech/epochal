use gtk4::prelude::*;
use gtk4::glib;
use gtk4::{Application, ApplicationWindow};
use libadwaita as adw;
use std::env;
use crate::tray::TrayAction;

mod tray;
use tray::TrayManager;

const APP_ID: &str = "com.korora.Epochal";

fn main() -> glib::ExitCode {
    // Create a new GTK application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Ensure libadwaita is initialized now that GTK is ready
    let _ = adw::init();

    // Initialize system tray (runtime-toggle with EPOCHAL_DISABLE_TRAY=1)
    let enable_tray = env::var("EPOCHAL_DISABLE_TRAY").is_err();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Epochal")
        .default_width(800)
        .default_height(600)
        .hide_on_close(true)
        .build();

    // Bridge tray events from worker threads to GTK main via idle_add
    use glib::ControlFlow::{Continue, Break};
    let (tx, rx) = std::sync::mpsc::channel::<TrayAction>();
    let app_for_actions = app.clone();
    let win_for_actions = window.clone();
    glib::idle_add_local(move || {
        match rx.try_recv() {
            Ok(TrayAction::ShowWindow) => {
                win_for_actions.present();
                Continue
            }
            Ok(TrayAction::Quit) => {
                app_for_actions.quit();
                Break
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => Continue,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Break,
        }
    });

    let _tray_manager = if enable_tray {
        match TrayManager::new(tx.clone()) {
            Ok(tray) => {
                println!("System tray icon created successfully");
                Some(tray)
            },
            Err(e) => {
                eprintln!("Tray init failed (continuing without tray): {}", e);
                None
            }
        }
    } else {
        println!("System tray disabled via EPOCHAL_DISABLE_TRAY");
        None
    };

    // Create the main content
    let header_bar = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Epochal", "GTK4 + Blueprint UI"))
        .build();

    let content = adw::StatusPage::builder()
        .title("Welcome to Epochal!")
        .description("Your GTK4 application with Blueprint UI and system tray icon")
        .icon_name("application-x-executable")
        .build();

    // Set up the window
    window.set_titlebar(Some(&header_bar));
    window.set_child(Some(&content));

    // Present window
    window.present();

    

    // Keep tray manager alive for the lifetime of the application
    if let Some(tray) = _tray_manager {
        // Store the tray manager in application data to keep it alive
        unsafe {
            app.set_data("tray_manager", tray);
        }
    }
}
