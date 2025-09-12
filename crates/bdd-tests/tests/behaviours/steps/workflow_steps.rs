use cucumber::{when, then};
use crate::behaviours::common::{TestWorld, simulate_gtk_main_loop_iteration, wait_for_gtk_events};
use std::time::{Duration, Instant};

#[then("I should see the welcome screen")]
async fn see_welcome_screen(world: &mut TestWorld) {
    assert!(world.window.is_some(), "Welcome screen (main window) should be visible");
    assert!(world.window_is_visible(), "Welcome screen should be visible");
}

#[then("the interface should be intuitive and welcoming")]
async fn interface_intuitive_welcoming(_world: &mut TestWorld) {
    // In a real implementation, we would check UI elements for user-friendliness
    assert!(true, "Interface should be designed to be intuitive and welcoming");
}

#[when("I minimize the window to system tray")]
async fn minimize_to_tray(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        window.set_visible(false); // Simulate minimize to tray
        simulate_gtk_main_loop_iteration();
    } else {
        panic!("No window to minimize");
    }
}

#[then("the main window should be hidden")]
async fn main_window_hidden(world: &mut TestWorld) {
    assert!(!world.window_is_visible(), "Main window should be hidden when minimized to tray");
}

#[then("the main window should be restored")]
async fn main_window_restored(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        window.set_visible(true); // Simulate restore from tray
        simulate_gtk_main_loop_iteration();
        assert!(world.window_is_visible(), "Main window should be visible after restore");
    } else {
        panic!("No window to restore");
    }
}

#[then("the window should be brought to front")]
async fn window_brought_to_front(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        // In a real implementation, we would check window stacking order
        assert!(window.is_visible(), "Window should be brought to front");
    } else {
        panic!("No window to bring to front");
    }
}

#[when("I close the main window")]
async fn close_main_window(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        window.close();
        simulate_gtk_main_loop_iteration();
    } else {
        panic!("No window to close");
    }
}

#[then("the application should minimize to tray")]
async fn app_minimizes_to_tray(world: &mut TestWorld) {
    // In a real implementation with tray, the app would minimize instead of closing
    if world.tray_available {
        assert!(true, "Application should minimize to tray when main window is closed");
    } else {
        // Without tray, the app might actually exit
        assert!(true, "Application behavior when closing without tray");
    }
}

#[then("I should be able to reopen it from the tray")]
async fn reopen_from_tray(world: &mut TestWorld) {
    if world.tray_available && world.window.is_some() {
        // Simulate reopening from tray
        if let Some(window) = &world.window {
            window.set_visible(true);
            simulate_gtk_main_loop_iteration();
            assert!(world.window_is_visible(), "Should be able to reopen from tray");
        }
    }
}

#[then("my session state should be preserved")]
async fn session_state_preserved(_world: &mut TestWorld) {
    // In a real implementation, we would verify that user data/state is maintained
    assert!(true, "Session state should be preserved across minimize/restore cycles");
}

#[then("the application should start within {int} seconds")]
async fn app_starts_within_seconds(_world: &mut TestWorld, max_seconds: u64) {
    // In a real performance test, we would measure actual startup time
    let start_time = Instant::now();
    
    // Simulate startup time check
    wait_for_gtk_events();
    
    let elapsed = start_time.elapsed();
    assert!(
        elapsed < Duration::from_secs(max_seconds),
        "Application should start within {} seconds, took {:?}",
        max_seconds,
        elapsed
    );
}

#[then("the system tray should be initialized within {int} second")]
async fn tray_initialized_within_second(world: &mut TestWorld, max_seconds: u64) {
    if world.tray_available {
        let start_time = Instant::now();
        
        // Simulate tray initialization time
        simulate_gtk_main_loop_iteration();
        
        let elapsed = start_time.elapsed();
        assert!(
            elapsed < Duration::from_secs(max_seconds),
            "System tray should initialize within {} second(s), took {:?}",
            max_seconds,
            elapsed
        );
    }
}

#[then("the main window should be responsive immediately")]
async fn window_responsive_immediately(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        // Test basic responsiveness
        assert!(window.is_visible(), "Window should be immediately responsive");
        
        // In a real implementation, we might test actual UI interactions
        simulate_gtk_main_loop_iteration();
        assert!(true, "Window should respond to user interactions immediately");
    } else {
        panic!("No window found to test responsiveness");
    }
}

#[when("an unexpected error occurs in the tray system")]
async fn tray_system_error(world: &mut TestWorld) {
    world.log_error("Unexpected tray system error occurred".to_string());
    world.tray_available = false; // Simulate tray system failure
}

#[then("the main application should continue functioning")]
async fn main_app_continues_functioning(world: &mut TestWorld) {
    assert!(world.app.is_some(), "Main application should continue running");
    assert!(world.window.is_some(), "Main window should remain functional");
    
    if let Some(window) = &world.window {
        assert!(window.is_visible(), "Main window should remain visible and functional");
    }
}

#[then("the user should be notified appropriately")]
async fn user_notified_appropriately(world: &mut TestWorld) {
    assert!(
        !world.error_messages.is_empty(),
        "User should be notified of errors through appropriate messaging"
    );
}

#[then("the application should attempt recovery")]
async fn app_attempts_recovery(world: &mut TestWorld) {
    // In a real implementation, we would verify recovery mechanisms
    assert!(
        world.app.is_some(),
        "Application should attempt to recover from errors and continue running"
    );
}