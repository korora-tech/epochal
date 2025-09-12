Feature: UI Interactions
  As a user
  I want to interact with the Epochal interface
  So that I can efficiently manage my calendar and tasks

  Background:
    Given the GTK environment is initialized
    And the Epochal application is running

  @ui @smoke
  Scenario: Main window displays welcome content
    Then I should see the header bar with title "Epochal"
    And I should see the subtitle "GTK4 + Blueprint UI"
    And I should see the welcome message "Welcome to Epochal!"
    And I should see the description containing "system tray icon"

  @ui @blueprint
  Scenario: Blueprint UI components are loaded
    When I examine the main window layout
    Then the header bar should be an Adwaita HeaderBar
    And the content should be an Adwaita StatusPage
    And the window should use Adwaita styling

  @ui @responsive
  Scenario: Window responsiveness
    When I resize the window to 400x300
    Then the content should remain properly laid out
    And text should remain readable
    When I resize the window to 1200x800
    Then the content should scale appropriately

  @accessibility @ui
  Scenario: Accessibility features
    When I examine the UI accessibility
    Then all interactive elements should have proper labels
    And keyboard navigation should work correctly
    And screen reader compatibility should be maintained

  @theming @ui
  Scenario: Theme support
    Given the application supports system theming
    When the system theme changes to dark mode
    Then the application should reflect the theme change
    And all UI elements should use appropriate colors