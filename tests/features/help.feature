Feature: Help

  Scenario: If the user asks for help
    Given an empty directory
    When the user executes the tool like
      """
      git-changed -h
      """
    Then it fails with the following error
      """
      Lists files modified since branching off

      Usage: git-changed [OPTIONS] [glob]

      Arguments:
        [glob]  Use this glob to filter files [default: **/*]

      Options:
        -0, --print0       Use NUL characters as separators (character code 0)
        -b, --base <base>  Use this commit instead of the default branch
        -h, --help         Print help information
      """
