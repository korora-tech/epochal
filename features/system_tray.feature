Feature: System Tray Integration
  As a user
  I want Epochal to provide a system tray icon
  So that I can quickly access it and minimize it to the background

  Background:
    Given the GTK environment is initialized
    And the system tray is available

  @tray @integration
  Scenario: System tray icon creation
    When I launch the Epochal application
    Then a system tray icon should be created
    And the tray manager should be stored in application data

  @tray @ui
  Scenario: Tray icon persistence
    Given the Epochal application is running
    And the system tray icon is visible
    When I minimize the main window
    Then the tray icon should remain visible
    And the application should continue running in background

  @tray @error-handling
  Scenario: Tray icon creation failure
    Given the system tray service is unavailable
    When I launch the Epochal application
    Then the TrayManager creation should fail gracefully
    And an error message should be displayed
    But the application should continue running normally

  @tray @lifecycle
  Scenario: Tray icon cleanup on exit
    Given the Epochal application is running with system tray
    When the application is terminated
    Then the system tray icon should be removed
    And no tray-related processes should remain