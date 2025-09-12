Feature: User Workflows
  As a user
  I want to perform common calendar tasks
  So that I can effectively manage my schedule

  Background:
    Given the GTK environment is initialized

  @workflow @smoke
  Scenario: First-time user experience
    When I launch Epochal for the first time
    Then I should see the welcome screen
    And I should see the "Welcome to Epochal!" message
    And the interface should be intuitive and welcoming

  @workflow @window-management
  Scenario: Window management workflow
    Given the Epochal application is running
    When I minimize the window to system tray
    Then the main window should be hidden
    And the tray icon should remain visible
    When I click on the tray icon
    Then the main window should be restored
    And the window should be brought to front

  @workflow @multitasking
  Scenario: Running in background
    Given Epochal is running with system tray
    When I close the main window
    Then the application should minimize to tray
    And I should be able to reopen it from the tray
    And my session state should be preserved

  @workflow @startup
  Scenario: Quick startup workflow
    When I launch Epochal
    Then the application should start within 3 seconds
    And the system tray should be initialized within 1 second
    And the main window should be responsive immediately

  @workflow @error-recovery
  Scenario: Graceful error handling
    Given Epochal is running normally
    When an unexpected error occurs in the tray system
    Then the main application should continue functioning
    And the user should be notified appropriately
    And the application should attempt recovery