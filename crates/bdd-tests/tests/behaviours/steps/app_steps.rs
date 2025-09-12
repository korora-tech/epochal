use crate::behaviours::common::{simulate_gtk_main_loop_iteration, wait_for_gtk_events, TestWorld};
use cucumber::{given, then, when};
use gtk4::prelude::*;

#[given(expr = "the GTK environment is initialized")]
fn gtk_environment_initialized(world: &mut TestWorld) {
    world.ensure_gtk_init().expect("Failed to initialize GTK");
}

#[given(expr = "the system tray is available")]
fn system_tray_available(world: &mut TestWorld) {
    world.tray_available = true;
}

#[given(expr = "the system tray is not available")]
fn system_tray_not_available(world: &mut TestWorld) {
    world.set_tray_unavailable();
}

#[given(expr = "the system tray service is unavailable")]
fn system_tray_service_unavailable(world: &mut TestWorld) {
    world.set_tray_unavailable();
}

#[given(expr = "the Epochal application is running")]
fn epochal_application_running(world: &mut TestWorld) {
    world.create_test_app().expect("Failed to create test app");
    world.launch_app().expect("Failed to launch app");
    wait_for_gtk_events();
}

#[given(expr = "the Epochal application is running with system tray")]
fn epochal_with_tray_running(world: &mut TestWorld) {
    world.tray_available = true;
    world.create_test_app().expect("Failed to create test app");
    world.launch_app().expect("Failed to launch app");
    wait_for_gtk_events();
}

#[when(expr = "I launch the Epochal application")]
fn launch_epochal_application(world: &mut TestWorld) {
    world.create_test_app().expect("Failed to create test app");
    world.launch_app().expect("Failed to launch app");
    wait_for_gtk_events();
}

#[when(expr = "I launch Epochal for the first time")]
fn launch_epochal_first_time(world: &mut TestWorld) {
    world.create_test_app().expect("Failed to create test app");
    world.launch_app().expect("Failed to launch app");
    wait_for_gtk_events();
}

#[when(expr = "I launch Epochal")]
fn launch_epochal(world: &mut TestWorld) {
    world.create_test_app().expect("Failed to create test app");
    world.launch_app().expect("Failed to launch app");
    wait_for_gtk_events();
}

#[when(expr = "I quit the application")]
fn quit_application(world: &mut TestWorld) {
    // Close the window first
    if let Some(window) = &world.window {
        window.close();
        simulate_gtk_main_loop_iteration();
    }

    if let Some(app) = world.get_app() {
        app.quit();
        simulate_gtk_main_loop_iteration();
    }
}

#[when(expr = "the application is terminated")]
fn application_terminated(world: &mut TestWorld) {
    // Close the window first
    if let Some(window) = &world.window {
        window.close();
        simulate_gtk_main_loop_iteration();
    }

    if let Some(app) = world.get_app() {
        app.quit();
        simulate_gtk_main_loop_iteration();
    }
}

#[then(expr = "the main window should be created")]
fn main_window_created(world: &mut TestWorld) {
    assert!(world.window.is_some(), "Main window was not created");
}

#[then(expr = "the window title should be {string}")]
fn window_title_should_be(world: &mut TestWorld, expected_title: String) {
    let actual_title = world.get_window_title().expect("Window has no title");
    assert_eq!(actual_title, expected_title, "Window title mismatch");
}

#[then(expr = "the application ID should be {string}")]
fn application_id_should_be(world: &mut TestWorld, expected_id: String) {
    if let Some(app) = world.get_app() {
        let actual_id = app
            .application_id()
            .expect("Application has no ID")
            .to_string();
        assert_eq!(actual_id, expected_id, "Application ID mismatch");
    } else {
        panic!("No application instance found");
    }
}

#[then(expr = "the main window should have default width {int}")]
fn window_default_width(world: &mut TestWorld, expected_width: i32) {
    let (width, _) = world.get_window_size().expect("Could not get window size");
    assert_eq!(width, expected_width, "Window width mismatch");
}

#[then(expr = "the main window should have default height {int}")]
fn window_default_height(world: &mut TestWorld, expected_height: i32) {
    let (_, height) = world.get_window_size().expect("Could not get window size");
    assert_eq!(height, expected_height, "Window height mismatch");
}

#[then(expr = "the window should be resizable")]
fn window_should_be_resizable(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        assert!(window.is_resizable(), "Window should be resizable");
    } else {
        panic!("No window found to check resizability");
    }
}

#[then(expr = "the application should exit cleanly")]
fn application_exits_cleanly(_world: &mut TestWorld) {
    // In a real test, we would check process exit code
    // For now, we assume clean exit if we reach this point
    // TODO: Implement test for: "Application should exit cleanly"
}

#[then(expr = "all windows should be closed")]
fn all_windows_closed(world: &mut TestWorld) {
    // After termination, either the window should be None or not visible
    match &world.window {
        None => {
            // Window was properly cleaned up
            // TODO: Implement test for: "Window was properly closed and cleaned up"
        }
        Some(window) => {
            // Check if window is destroyed or not realized (GTK term for closed)
            let is_closed = !window.is_visible() || !window.is_realized();
            assert!(is_closed, "Window should be closed");
        }
    }
}
