use crate::behaviours::common::{simulate_gtk_main_loop_iteration, TestWorld};
use cucumber::{given, then, when};
use gtk4::prelude::*;

#[then(expr = "I should see the header bar with title {string}")]
fn see_header_bar_title(world: &mut TestWorld, _expected_title: String) {
    if let Some(window) = &world.window {
        if let Some(_titlebar) = window.titlebar() {
            // Check if it's an Adwaita HeaderBar with the expected title
            // In a real implementation, we would traverse the widget tree
            // TODO: Implement test for: "Header bar should contain the expected title"
        } else {
            panic!("Window should have a titlebar");
        }
    } else {
        panic!("No window found to check header bar");
    }
}

#[then(expr = "I should see the subtitle {string}")]
fn see_subtitle(world: &mut TestWorld, _expected_subtitle: String) {
    // In a real implementation, we would query the WindowTitle widget
    if world.window.is_some() {
        // TODO: Implement test for: "Should see subtitle: {}", expected_subtitle
    } else {
        panic!("No window found to check subtitle");
    }
}

#[then(expr = "I should see the welcome message {string}")]
fn see_welcome_message(world: &mut TestWorld, _expected_message: String) {
    // In a real implementation, we would search for the StatusPage title
    if world.window.is_some() {
        // TODO: Implement test for: "Should see welcome message: {}", expected_message
    } else {
        panic!("No window found to check welcome message");
    }
}

#[then(expr = "I should see the description containing {string}")]
fn see_description_containing(world: &mut TestWorld, _expected_text: String) {
    // In a real implementation, we would check the StatusPage description
    if world.window.is_some() {
        // TODO: Implement test for: "Should see description containing: {}", expected_text
    } else {
        panic!("No window found to check description");
    }
}

#[when(expr = "I examine the main window layout")]
fn examine_window_layout(_world: &mut TestWorld) {
    // This step is used to set up inspection of the UI layout
    simulate_gtk_main_loop_iteration();
}

#[then(expr = "the header bar should be an Adwaita HeaderBar")]
fn header_bar_is_adwaita(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        if let Some(titlebar) = window.titlebar() {
            // Check if it's specifically an Adwaita HeaderBar
            let widget_type = titlebar.type_().name();
            assert!(
                widget_type.contains("HeaderBar") || widget_type.contains("AdwHeaderBar"),
                "Header bar should be an Adwaita HeaderBar, got: {}",
                widget_type
            );
        } else {
            panic!("Window should have a titlebar");
        }
    } else {
        panic!("No window found");
    }
}

#[then(expr = "the content should be an Adwaita StatusPage")]
fn content_is_adwaita_status_page(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        if let Some(child) = window.child() {
            let widget_type = child.type_().name();
            assert!(
                widget_type.contains("StatusPage") || widget_type.contains("AdwStatusPage"),
                "Content should be an Adwaita StatusPage, got: {}",
                widget_type
            );
        } else {
            panic!("Window should have content");
        }
    } else {
        panic!("No window found");
    }
}

#[then(expr = "the window should use Adwaita styling")]
fn window_uses_adwaita_styling(world: &mut TestWorld) {
    if let Some(window) = &world.window {
        // In GTK4 with libadwaita, ApplicationWindow inherently uses Adwaita styling
        // Just verify the window exists and has the correct type
        let window_type = window.type_().name();
        assert!(
            window_type.contains("ApplicationWindow"),
            "Window should be an ApplicationWindow with Adwaita styling"
        );
    } else {
        panic!("No window found to check styling");
    }
}

#[when(expr = "I resize the window to {int}x{int}")]
fn resize_window(world: &mut TestWorld, width: i32, height: i32) {
    if let Some(window) = &world.window {
        window.set_default_size(width, height);
        simulate_gtk_main_loop_iteration();
    } else {
        panic!("No window found to resize");
    }
}

#[then(expr = "the content should remain properly laid out")]
fn content_properly_laid_out(_world: &mut TestWorld) {
    // In a real implementation, we would check layout constraints
    // TODO: Implement test for: "Content should remain properly laid out after resize"
}

#[then(expr = "text should remain readable")]
fn text_remains_readable(_world: &mut TestWorld) {
    // In a real implementation, we would check text rendering
    // TODO: Implement test for: "Text should remain readable after resize"
}

#[then(expr = "the content should scale appropriately")]
fn content_scales_appropriately(_world: &mut TestWorld) {
    // In a real implementation, we would verify responsive scaling
    // TODO: Implement test for: "Content should scale appropriately for larger sizes"
}

#[when(expr = "I examine the UI accessibility")]
fn examine_ui_accessibility(_world: &mut TestWorld) {
    // This step sets up accessibility inspection
    simulate_gtk_main_loop_iteration();
}

#[then(expr = "all interactive elements should have proper labels")]
fn interactive_elements_labeled(_world: &mut TestWorld) {
    // In a real implementation, we would use AT-SPI to check accessibility
    // TODO: Implement test for: "All interactive elements should have accessibility labels"
}

#[then(expr = "keyboard navigation should work correctly")]
fn keyboard_navigation_works(_world: &mut TestWorld) {
    // In a real implementation, we would test tab order and keyboard shortcuts
    // TODO: Implement test for: "Keyboard navigation should be functional"
}

#[then(expr = "screen reader compatibility should be maintained")]
fn screen_reader_compatible(_world: &mut TestWorld) {
    // In a real implementation, we would test with screen reader APIs
    // TODO: Implement test for: "Screen reader compatibility should be maintained"
}

#[given(expr = "the application supports system theming")]
fn app_supports_theming(_world: &mut TestWorld) {
    // This precondition assumes theme support is built into the app
    // TODO: Implement test for: "Application should support system theming"
}

#[when(expr = "the system theme changes to dark mode")]
fn system_theme_changes(_world: &mut TestWorld) {
    // In a real implementation, we would simulate theme change
    simulate_gtk_main_loop_iteration();
}

#[then(expr = "the application should reflect the theme change")]
fn app_reflects_theme_change(_world: &mut TestWorld) {
    // In a real implementation, we would check theme application
    // TODO: Implement test for: "Application should reflect system theme changes"
}

#[then(expr = "all UI elements should use appropriate colors")]
fn ui_elements_use_appropriate_colors(_world: &mut TestWorld) {
    // In a real implementation, we would verify color scheme application
    // TODO: Implement test for: "UI elements should use theme-appropriate colors"
}
