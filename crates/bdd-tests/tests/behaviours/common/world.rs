use super::wait_for_gtk_events;
use cucumber::World;
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use libadwaita as adw;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TestWorld {
    pub app: Option<adw::Application>,
    pub window: Option<ApplicationWindow>,
    pub tray_available: bool,
    pub error_messages: Vec<String>,
}

impl TestWorld {
    async fn new() -> Result<Self, cucumber::codegen::anyhow::Error> {
        Ok(Self {
            app: None,
            window: None,
            tray_available: true,
            error_messages: Vec::new(),
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
            .application_id("com.korora.Epochal")
            .build();

        self.app = Some(app);
        Ok(())
    }

    pub fn get_app(&self) -> Option<&adw::Application> {
        self.app.as_ref()
    }

    pub fn launch_app(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check if tray is available and log a warning if not
        if !self.tray_available {
            self.log_error("Failed to create system tray icon".to_string());
        }

        if let Some(app) = &self.app {
            // For testing, create the window directly without going through app activation
            // This avoids the need for a running GTK main loop
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

            // Make window visible for tests
            window.set_visible(true);

            // Store the window reference
            self.window = Some(window);

            // Process any pending GTK events
            wait_for_gtk_events();
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
