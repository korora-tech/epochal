use cucumber::World;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use libadwaita as adw;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {
    pub app: Option<adw::Application>,
    pub window: Option<ApplicationWindow>,
    pub widgets: HashMap<String, gtk4::Widget>,
    pub tray_available: bool,
    pub error_messages: Vec<String>,
    pub runtime: Option<tokio::runtime::Handle>,
}

impl TestWorld {
    async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            app: None,
            window: None,
            widgets: HashMap::new(),
            tray_available: true,
            error_messages: Vec::new(),
            runtime: None,
        })
    }

    pub fn ensure_gtk_init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !gtk4::is_initialized() {
            adw::init().map_err(|_| "Failed to initialize Adwaita")?;
        }
        Ok(())
    }

    pub fn create_test_app(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_gtk_init()?;
        
        let app = adw::Application::builder()
            .application_id("com.korora.Epochal.test")
            .build();
        
        self.app = Some(app);
        Ok(())
    }

    pub fn get_app(&self) -> Option<&adw::Application> {
        self.app.as_ref()
    }

    pub fn launch_app(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(app) = &self.app {
            let app_clone = app.clone();
            let window_ref = Arc::new(Mutex::new(None));
            let window_ref_clone = window_ref.clone();

            app.connect_activate(move |app| {
                // Create window similar to main.rs but for testing
                let window = ApplicationWindow::builder()
                    .application(app)
                    .title("Epochal")
                    .default_width(800)
                    .default_height(600)
                    .build();

                let header_bar = adw::HeaderBar::builder()
                    .title_widget(&adw::WindowTitle::new("Epochal", "GTK4 + Blueprint UI"))
                    .build();

                let content = adw::StatusPage::builder()
                    .title("Welcome to Epochal!")
                    .description("Your GTK4 application with Blueprint UI and system tray icon")
                    .icon_name("application-x-executable")
                    .build();

                window.set_titlebar(Some(&header_bar));
                window.set_child(Some(&content));

                // Store window reference for testing
                if let Ok(mut window_guard) = window_ref_clone.lock() {
                    *window_guard = Some(window.clone());
                }

                window.present();
            });

            // Simulate app activation
            app.activate();
            
            // Get the window reference
            if let Ok(window_guard) = window_ref.lock() {
                if let Some(window) = window_guard.clone() {
                    self.window = Some(window);
                }
            }
        }
        Ok(())
    }

    pub fn set_tray_unavailable(&mut self) {
        self.tray_available = false;
    }

    pub fn log_error(&mut self, error: String) {
        self.error_messages.push(error);
    }

    pub fn get_last_error(&self) -> Option<&String> {
        self.error_messages.last()
    }

    pub fn window_is_visible(&self) -> bool {
        self.window
            .as_ref()
            .map(|w| w.is_visible())
            .unwrap_or(false)
    }

    pub fn get_window_title(&self) -> Option<String> {
        self.window
            .as_ref()
            .and_then(|w| w.title())
            .map(|s| s.to_string())
    }

    pub fn get_window_size(&self) -> Option<(i32, i32)> {
        self.window
            .as_ref()
            .map(|w| (w.default_width(), w.default_height()))
    }
}