Feature: Default branch from the git config

  Background: A feature branch with a committed file
    Given an empty directory
    When the user executes the command: "git init --initial-branch=main"
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"
    And the user executes the command: "git switch --create feature"
    And the user executes the command: "touch README.md"
    And the user executes the command: "git add README.md"
    And the user executes the command: "git commit --message 'Adding README'"

  Scenario: It uses the default branch from the repository config
    When the user executes the command: "git config changed.defaultBranch feature"
    And the user executes the tool like: "git-changed"
    Then it succeeds with the following output
      """
      """
