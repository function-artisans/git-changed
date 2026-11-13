Feature: Compare with a given base

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

  Scenario: It uses a branch with a name that looks like a commit ID
    When the user executes the command: "git branch cafe main"
    And the user executes the tool like: "git-changed --base cafe"
    Then it succeeds with the following output
      """
      README.md
      """

  Scenario: It uses a tag
    When the user executes the command: "git tag start main"
    And the user executes the tool like: "git-changed --base start"
    Then it succeeds with the following output
      """
      README.md
      """

  Scenario: It uses a relative revision
    When the user executes the tool like: "git-changed --base HEAD~1"
    Then it succeeds with the following output
      """
      README.md
      """
