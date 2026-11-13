Feature: Run the tool in a subdirectory

  Background: A git repository with a new file in a subdirectory
    Given an empty directory
    When the user executes the command: "git init --initial-branch=main"
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"
    And the user executes the command: "mkdir src"
    And the user executes the command: "touch README.md src/main.rs"
    And the user executes the command: "git add README.md src/main.rs"

  Scenario: It displays the changes of the repository
    When the user executes the tool in "src" like: "git-changed *.rs"
    Then it succeeds with the following output
      """
      src/main.rs
      """

  Scenario: It finds the existing files from the root of the repository
    When the user executes the tool in "src" like: "git-changed -e *.md"
    Then it succeeds with the following output
      """
      README.md
      """
