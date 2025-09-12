Feature: Application Lifecycle
  As a user
  I want Epochal to start and stop reliably
  So that I can trust it as my daily calendar application

  Background:
    Given the GTK environment is initialized

  @smoke @headless
  Scenario: Application starts successfully
    When I launch the Epochal application
    Then the main window should be created
    And the window title should be "Epochal"
    And the application ID should be "com.korora.Epochal"

  @integration @headless
  Scenario: System tray initialization
    When I launch the Epochal application
    Then the system tray manager should be created
    And the tray icon should be visible in supported environments

  @error-handling @headless
  Scenario: Graceful system tray failure
    Given the system tray is not available
    When I launch the Epochal application
    Then the application should continue running
    And a warning message should be logged about tray failure
    And the main window should still be functional

  @ui @integration
  Scenario: Window properties are correct
    When I launch the Epochal application
    Then the main window should have default width 800
    And the main window should have default height 600
    And the window should be resizable

  @shutdown @integration
  Scenario: Clean application shutdown
    Given the Epochal application is running
    When I quit the application
    Then the system tray icon should be removed
    And all windows should be closed
    And the application should exit cleanly