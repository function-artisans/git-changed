Feature: Untracked files

  Background: A git repository with an untracked directory
    Given an empty directory
    When the user executes the command: "git init --initial-branch=main"
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"
    And the user executes the command: "git update-ref refs/remotes/origin/main main"
    And the user executes the command: "mkdir docs"
    And the user executes the command: "touch docs/guide.md"

  Scenario: It displays the files in the untracked directory
    When the user executes the tool like: "git-changed"
    Then it succeeds with the following output
      """
      docs/guide.md
      """
