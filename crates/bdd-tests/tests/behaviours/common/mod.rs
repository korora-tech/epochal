pub mod world;

pub use world::TestWorld;

// Test utilities
pub fn wait_for_gtk_events() {
    while gtk4::glib::MainContext::default().pending() {
        gtk4::glib::MainContext::default().iteration(false);
    }
}

pub fn simulate_gtk_main_loop_iteration() {
    let context = gtk4::glib::MainContext::default();
    while context.pending() {
        context.iteration(false);
    }
    std::thread::sleep(std::time::Duration::from_millis(10));
}
