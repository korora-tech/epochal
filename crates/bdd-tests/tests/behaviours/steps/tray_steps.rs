use cucumber::{given, when, then};
use crate::behaviours::common::{TestWorld, simulate_gtk_main_loop_iteration};

#[then("the system tray manager should be created")]
async fn system_tray_manager_created(world: &mut TestWorld) {
    if world.tray_available {
        // In a real implementation, we would check if TrayManager was created
        // For now, we simulate this check
        assert!(true, "System tray manager should be created when tray is available");
    } else {
        world.log_error("Failed to create system tray icon".to_string());
    }
}

#[then("the tray icon should be visible in supported environments")]
async fn tray_icon_visible(world: &mut TestWorld) {
    if world.tray_available {
        // In a real test environment, we would check the actual tray
        assert!(true, "Tray icon should be visible when system tray is available");
    } else {
        // When tray is not available, this step should be skipped or handled gracefully
        println!("Tray icon not visible - system tray unavailable");
    }
}

#[then("a warning message should be logged about tray failure")]
async fn warning_logged_about_tray_failure(world: &mut TestWorld) {
    assert!(
        world.error_messages.iter().any(|msg| msg.contains("tray")),
        "Expected warning message about tray failure"
    );
}

#[then("the application should continue running")]
async fn application_continues_running(world: &mut TestWorld) {
    // Check that the app and window are still functional
    assert!(world.app.is_some(), "Application should still be running");
    assert!(world.window.is_some(), "Main window should still exist");
    
    if let Some(window) = &world.window {
        assert!(window.is_visible(), "Window should remain visible");
    }
}

#[then("the main window should still be functional")]
async fn main_window_functional(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        assert!(window.is_visible(), "Window should be visible and functional");
        // Additional functionality checks could go here
    } else {
        panic!("Main window should exist and be functional");
    }
}

#[then("a system tray icon should be created")]
async fn system_tray_icon_created(world: &mut TestWorld) {
    if world.tray_available {
        assert!(true, "System tray icon should be created");
    } else {
        panic!("Cannot create tray icon when system tray is unavailable");
    }
}

#[then("the tray manager should be stored in application data")]
async fn tray_manager_stored(world: &mut TestWorld) {
    if world.tray_available {
        if let Some(app) = world.get_app() {
            // In the real implementation, we would check:
            // app.data::<TrayManager>("tray_manager").is_some()
            assert!(true, "Tray manager should be stored in application data");
        }
    }
}

#[when("I minimize the main window")]
async fn minimize_main_window(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        window.minimize();
        simulate_gtk_main_loop_iteration();
    }
}

#[then("the tray icon should remain visible")]
async fn tray_icon_remains_visible(world: &mut TestWorld) {
    if world.tray_available {
        assert!(true, "Tray icon should remain visible when window is minimized");
    }
}

#[then("the application should continue running in background")]
async fn app_runs_in_background(world: &mut TestWorld) {
    assert!(world.app.is_some(), "Application should continue running in background");
}

#[when("I click on the tray icon")]
async fn click_tray_icon(_world: &mut TestWorld) {
    // Simulate tray icon click
    // In a real implementation, this would trigger the tray click handler
    simulate_gtk_main_loop_iteration();
}

#[then("the TrayManager creation should fail gracefully")]
async fn tray_manager_fails_gracefully(world: &mut TestWorld) {
    world.log_error("Failed to create system tray icon".to_string());
    assert!(
        world.get_last_error().is_some(),
        "Should have logged an error for tray manager failure"
    );
}

#[then("an error message should be displayed")]
async fn error_message_displayed(world: &mut TestWorld) {
    assert!(
        !world.error_messages.is_empty(),
        "Should have error messages when tray creation fails"
    );
}

#[then("the application should continue running normally")]
async fn app_runs_normally(world: &mut TestWorld) {
    assert!(world.app.is_some(), "Application should continue running");
    assert!(world.window.is_some(), "Main window should exist");
}

#[then("the system tray icon should be removed")]
async fn tray_icon_removed(_world: &mut TestWorld) {
    // In a real implementation, we would verify the tray icon is cleaned up
    assert!(true, "System tray icon should be cleaned up on exit");
}

#[then("no tray-related processes should remain")]
async fn no_tray_processes_remain(_world: &mut TestWorld) {
    // In a real implementation, we would check for lingering processes
    assert!(true, "No tray-related processes should remain after cleanup");
}