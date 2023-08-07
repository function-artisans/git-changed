use cucumber::gherkin::Step;
use cucumber::given;
use cucumber::then;
use cucumber::when;
use cucumber::World;
use shlex::split;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use stdio_override::StdoutOverride;
use tempfile::NamedTempFile;
use uuid::Uuid;

#[path = "../src/main.rs"]
mod main;

fn capture_stdout<F: Fn()>(f: F) -> String {
    let temp_file = NamedTempFile::new().expect("Could not create temporary file");
    let captured_output = temp_file.into_temp_path().to_path_buf();

    let guard = StdoutOverride::override_file(captured_output.clone());

    f();

    drop(guard);

    fs::read_to_string(captured_output).expect("Could not read captured output")
}

fn get_doc_string(step: &Step) -> String {
    step.docstring
        .as_ref()
        .expect("Doc string needed!")
        .trim()
        .to_string()
}

#[derive(Debug, Default, World)]
pub struct GitWorld {
    directory: PathBuf,
    command: String,
}

#[given("an empty directory")]
fn init_directory(world: &mut GitWorld) {
    let dir = env::temp_dir();
    let uuid = Uuid::new_v4().to_string();

    let name = format!("git-changed-{}", &uuid);
    let directory = dir.join(name);

    fs::create_dir_all(directory.clone()).expect("Could not create directory!");

    world.directory = directory;
}

#[when(expr = "the user executes the command: {string}")]
fn execute_command(world: &mut GitWorld, command: String) {
    let command_parts = split(command.as_str()).expect("Could not parse command");

    let (program, args) = command_parts.split_at(1);

    let output = Command::new(program.first().expect("Could not get the command"))
        .current_dir(world.directory.clone())
        .args(args)
        .output()
        .expect("Failed to create the repository");

    assert!(output.status.success());
}

#[when(expr = "the user executes the tool like: {string}")]
fn execute(world: &mut GitWorld, command: String) {
    world.command = command;
}

#[then("it fails with the following error")]
fn assert_failure(world: &mut GitWorld, step: &Step) {
    let expected_error = get_doc_string(step);

    let result = main::from(world.command.clone());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string().trim(), expected_error);
}

#[then("it succeeds with the following output")]
fn assert_success(world: &mut GitWorld, step: &Step) {
    let expected_output = get_doc_string(step);

    let actual_output = capture_stdout(|| {
        env::set_current_dir(&world.directory).expect("Could not change to directory");

        main::from(world.command.clone()).expect("Could not execute");
    });

    assert_eq!(expected_output, actual_output.trim());
}

fn cleanup(world: &mut GitWorld) {
    fs::remove_dir_all(world.directory.clone()).expect("Could not remove directory");
}

fn main() {
    let world = GitWorld::cucumber()
        .after(|_feature, _rule, _scenario, _ev, world| {
            Box::pin(async {
                cleanup(world.expect("World is missing"));
            })
        })
        .run_and_exit("tests/features");

    futures::executor::block_on(world);
}
