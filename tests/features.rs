use cucumber::gherkin::Step;
use cucumber::given;
use cucumber::then;
use cucumber::when;
use cucumber::World;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use tempfile::TempDir;

fn main() {
    futures::executor::block_on(GitWorld::cucumber().run_and_exit("tests/features"));
}

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct GitWorld {
    home: TempDir,
    directory: TempDir,
    output: Option<Output>,
}

#[given("an empty directory")]
fn empty_directory(_world: &mut GitWorld) {
    // GitWorld::new creates the directory before the first step.
}

#[when(expr = "the user executes the command: {string}")]
fn run_command(world: &mut GitWorld, command: String) {
    let (program, args) = split_command(&command);

    let output = world.run(&program, &args);

    assert!(
        output.status.success(),
        "The command failed: {command}\n{}",
        text(&output.stderr)
    );
}

#[when(expr = "the user executes the tool like: {string}")]
fn run_tool(world: &mut GitWorld, command: String) {
    run_tool_in(world, ".".to_string(), command);
}

#[when(expr = "the user executes the tool in {string} like: {string}")]
fn run_tool_in(world: &mut GitWorld, subdirectory: String, command: String) {
    let (program, args) = split_command(&command);
    assert_eq!(program, "git-changed", "This step runs only git-changed");

    let output = world.run_in(&subdirectory, env!("CARGO_BIN_EXE_git-changed"), &args);

    world.output = Some(output);
}

#[then("it fails with the following error")]
fn assert_failure(world: &mut GitWorld, step: &Step) {
    let output = world.tool_output();

    assert!(!output.status.success(), "The tool did not fail");
    assert_eq!(text(&output.stderr), expected_text(step));
}

#[then("it succeeds with the following output")]
fn assert_success(world: &mut GitWorld, step: &Step) {
    let output = world.tool_output();

    assert!(
        output.status.success(),
        "The tool failed: {}",
        text(&output.stderr)
    );
    assert_eq!(text(&output.stdout), expected_text(step));
}

impl GitWorld {
    fn new() -> Self {
        let home = TempDir::new().expect("Cannot create the home directory");
        write_global_config(home.path());

        Self {
            home,
            directory: TempDir::new().expect("Cannot create the work directory"),
            output: None,
        }
    }

    fn run(&self, program: &str, args: &[String]) -> Output {
        self.run_in(".", program, args)
    }

    fn run_in(&self, subdirectory: &str, program: &str, args: &[String]) -> Output {
        let directory = self.directory.path().join(subdirectory);

        command_without_user_config(program, self.home.path(), &directory)
            .args(args)
            .output()
            .unwrap_or_else(|error| panic!("Cannot run {program}: {error}"))
    }

    fn tool_output(&self) -> &Output {
        self.output.as_ref().expect("The tool did not run")
    }
}

fn command_without_user_config(program: &str, home: &Path, directory: &Path) -> Command {
    let mut command = Command::new(program);

    // git and libgit2 read the global config from HOME and XDG_CONFIG_HOME.
    command
        .current_dir(directory)
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home)
        .env("GIT_CONFIG_NOSYSTEM", "1");

    command
}

fn write_global_config(home: &Path) {
    // libgit2 ignores GIT_CONFIG_NOSYSTEM. This value overrides the one in /etc/gitconfig.
    let config = "[init]\n\tdefaultBranch = main\n";

    fs::write(home.join(".gitconfig"), config).expect("Cannot write the global git config");
}

fn split_command(command: &str) -> (String, Vec<String>) {
    let mut parts =
        shlex::split(command).unwrap_or_else(|| panic!("Cannot parse the command: {command}"));
    assert!(!parts.is_empty(), "The command is empty");

    let program = parts.remove(0);

    (program, parts)
}

fn expected_text(step: &Step) -> String {
    let doc_string = step.docstring.as_ref().expect("The step has no doc string");

    doc_string.trim().to_string()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}
