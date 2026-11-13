Feature: Output

  Background: A git repository with untracked files
    Given an empty directory
    When the user executes the command: "git init --initial-branch=main"
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"
    And the user executes the command: "touch c.md a.md e.md b.md d.md"

  Scenario: It displays the files in sorted order
    When the user executes the tool like: "git-changed"
    Then it succeeds with the following output
      """
      a.md
      b.md
      c.md
      d.md
      e.md
      """

  Scenario: It stops without an error if the reader closes the output
    When the user executes the tool with a closed output like: "git-changed"
    Then it succeeds with the following output
      """
      """

  Scenario: It separates the files with NUL characters
    When the user executes the tool like: "git-changed -0"
    Then it succeeds with the following NUL-separated output
      """
      a.md
      b.md
      c.md
      d.md
      e.md
      """
