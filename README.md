# git-changed

## Description

This is a small tool that helps to list the files you have changed (added/modified/removed) in your git branch.

## Motivation

The author wrote this tool for a project with very slow tests and tools. A full run took too long for each small change. The author wanted to run the tests and tools only on the files of the current branch.

The tool lists these files:

- the files from the commits since the branch left `origin/main`
- the files with uncommitted changes
- the untracked files

The tool only prints the list. Send the list to another tool:

```bash
$ git-changed -e '**/*.rs' | xargs rustfmt
```

## Alternatives

- `git diff --name-only $(git merge-base origin/main HEAD)` lists the committed and the uncommitted changes. It does not list the untracked files. Add `git ls-files --others --exclude-standard` for those.
- [lint-staged](https://github.com/lint-staged/lint-staged) runs commands on the staged files before a commit. It needs Node.js.
- [lefthook](https://github.com/evilmartians/lefthook) is a git hook manager. It gives `{staged_files}` and `{push_files}` to the commands. A `lefthook` command can also call `git-changed`.
- [tj-actions/changed-files](https://github.com/tj-actions/changed-files) lists the changed files in GitHub Actions.

`git-changed` does not run commands and does not manage hooks. It only lists the files, so you can use it in a terminal, in a hook, or in CI.

## Install with Cargo

Install the tool from [crates.io](https://crates.io/crates/git-changed):

```bash
$ cargo install git-changed
```

## Install with Nix

This project is a Nix flake. It gives you the `git-changed` package, an overlay and an app.

Run the tool without an installation:

```bash
$ nix run github:function-artisans/git-changed
```

Add the flake to your own configuration:

```nix
{
  inputs.git-changed.url = "github:function-artisans/git-changed";
}
```

Then take the package for your system:

```nix
inputs.git-changed.packages.${system}.default
```

Or add the overlay and take `pkgs.git-changed`:

```nix
nixpkgs.overlays = [ inputs.git-changed.overlays.default ];
```

## Usage

To get the recent changes comparing to the default branch you can type:

```bash
$ git-changed
```

In a subdirectory, the tool prints the paths relative to that directory, like `git status`. The glob also matches the relative path:

```bash
$ cd src
$ git-changed '*.rs' | xargs rustfmt
```

### Default branch

The tool compares your branch with `origin/main`. It does not use your local `main` branch, because that branch can be old.

To use a different base, give the `--base` option:

```bash
$ git-changed --base development
```

Or you can set the configuration globally:

```bash
$ git config --global changed.defaultBranch development
```

Or only for the current repository:

```bash
$ git config changed.defaultBranch development
```

The value can be any revision, for example a branch, a tag, or `upstream/main`.

### Filtering files with glob pattern

```bash
$ git-changed '**/*.rs'
```

### Using `\0` as file separator

If you need to work with filenames that could contain spaces, then you can change the separator from `\n` to a `NUL` character

```bash
$ git-changed -0
```

### Filtering existing files only

Some tools may not handle files that no longer exist. With this option, you can easily filter out non-existing files, ensuring reliable processing.

```bash
$ git-changed -e
```

### Examples

Run only the specs of the branch. A new spec file is in the list before you run `git add`:

```bash
$ git-changed -e 'spec/**/*_spec.rb' | xargs bundle exec rspec
```

Lint the changed files. The `-e` option removes the deleted files, so the linter does not fail on a missing file:

```bash
$ git-changed -e -0 '**/*.py' | xargs -0 ruff check
```

Run the tests again each time that you save a changed file. [entr](https://github.com/eradman/entr) reads the list of files to watch from standard input:

```bash
$ git-changed -e '**/*.rs' | entr -c cargo test
```

Open all changed files in the editor:

```bash
$ vim $(git-changed -e)
```

Select one changed file with [fzf](https://github.com/junegunn/fzf) and open it:

```bash
$ vim "$(git-changed -e | fzf)"
```

Run a linter before each push with lefthook. The list contains all the work on the branch, also the work that you did not commit:

```yaml
pre-push:
  commands:
    rubocop:
      run: git-changed -e -0 '**/*.rb' | xargs -0 bundle exec rubocop
```

Compare with the `development` branch and compile each changed file:

```bash
$ git-changed -0 -b development '**/*.rs' | xargs -0 -n 1 rustc
```

## Dependencies

- [clap](https://docs.rs/clap/latest/clap/)
- [git2](https://docs.rs/git2/latest/git2/)
- [glob](https://docs.rs/glob/latest/glob/)
