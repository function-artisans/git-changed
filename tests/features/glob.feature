Feature: Glob

  Scenario: It rejects an invalid glob
    Given an empty directory
    When the user executes the tool like: "git-changed ["
    Then it fails with the following error
      """
      error: invalid value '[' for '[glob]': Pattern syntax error near position 0: invalid range pattern

      For more information, try '--help'.
      """
