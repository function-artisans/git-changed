Feature: Empty git repository

  Background: An empty git repository
    Given an empty directory
    When the user executes the command: "git init ."
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"

  Scenario: It displays nothing
    When the user executes the tool like: "git-changed"
    Then it succeeds with the following output
      """
      """
