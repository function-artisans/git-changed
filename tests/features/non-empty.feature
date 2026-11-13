Feature: Non-empty git repository

  Background: An empty git repository
    Given an empty directory
    When the user executes the command: "git init --initial-branch=main"
    And the user executes the command: "git config user.name 'John Doe'"
    And the user executes the command: "git config user.email 'jdoe@example.com'"
    And the user executes the command: "git commit --allow-empty --message 'Initial commit'"
    And the user executes the command: "touch README.md"
    And the user executes the command: "git add README.md"

    Rule: With uncommitted changes

      Scenario: It displays the changes
        When the user executes the tool like: "git-changed"
        Then it succeeds with the following output
          """
          README.md
          """

      Scenario: It does not display non-matching changes
        When the user executes the tool like: "git-changed *.txt"
        Then it succeeds with the following output
          """
          """

      Scenario: It display matching changes
        When the user executes the tool like: "git-changed *.md"
        Then it succeeds with the following output
          """
          README.md
          """

    Rule: With committed changes

      Background: With committed changes
        When the user executes the command: "git commit --message 'Adding README'"

      Scenario: It displays nothing if there are no changes
        When the user executes the tool like: "git-changed"
        Then it succeeds with the following output
          """
          """

      Rule: When the changed files does not exist anymore

        Background: With deleted files
          When the user executes the command: "rm README.md"

          Scenario: It displays the removed files
            When the user executes the tool like: "git-changed"
            Then it succeeds with the following output
              """
              README.md
              """

          Scenario: It filters for existing files when requested
            When the user executes the tool like: "git-changed -e"
            Then it succeeds with the following output
              """
              """

      Rule: With uncommitted changes

        Background: With uncommitted changes
          When the user executes the command: "dd if=/dev/random of=README.md count=1"

          Scenario: It displays the uncommitted changes
            When the user executes the tool like: "git-changed"
            Then it succeeds with the following output
              """
              README.md
              """

          Scenario: It does not display non-matching changes
            When the user executes the tool like: "git-changed *.txt"
            Then it succeeds with the following output
              """
              """

          Scenario: It display matching changes
            When the user executes the tool like: "git-changed *.md"
            Then it succeeds with the following output
              """
              README.md
              """
