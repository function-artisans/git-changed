Feature: Compare with origin/main by default

  Background: A git repository with a remote branch
    Given an empty directory
    When the user executes the command: "git init --initial-branch=main"
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"
    And the user executes the command: "git update-ref refs/remotes/origin/main main"

  Scenario: It displays the commits that are not pushed to origin/main
    When the user executes the command: "touch README.md"
    And the user executes the command: "git add README.md"
    And the user executes the command: "git commit --message 'Adding README'"
    And the user executes the tool like: "git-changed"
    Then it succeeds with the following output
      """
      README.md
      """

  Scenario: It ignores an old local main branch
    When the user executes the command: "git switch --create upstream"
    And the user executes the command: "touch LICENSE"
    And the user executes the command: "git add LICENSE"
    And the user executes the command: "git commit --message 'Adding LICENSE'"
    And the user executes the command: "git update-ref refs/remotes/origin/main upstream"
    And the user executes the command: "git switch --create feature"
    And the user executes the command: "touch README.md"
    And the user executes the command: "git add README.md"
    And the user executes the command: "git commit --message 'Adding README'"
    And the user executes the tool like: "git-changed"
    Then it succeeds with the following output
      """
      README.md
      """

  Scenario: It fails if origin/main does not exist
    When the user executes the command: "git update-ref -d refs/remotes/origin/main"
    And the user executes the tool like: "git-changed"
    Then it fails with the following error
      """
      Cannot find the commit of origin/main: revspec 'origin/main' not found; class=Reference (4); code=NotFound (-3)
      """
