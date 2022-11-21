extern crate git2;

use git2::BranchType;
use git2::DiffDelta;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use git2::Tree;
use std::collections::HashSet;

fn status_options() -> StatusOptions {
    let mut options = StatusOptions::default();

    options.include_untracked(true);
    options.exclude_submodules(true);

    options
}

fn uncommitted_changes(repo: &Repository) -> HashSet<String> {
    let mut options = status_options();

    let statuses = repo
        .statuses(Some(&mut options))
        .expect("Failed to get statuses");

    let iter = statuses
        .iter()
        .map(|entry| entry.path().expect("Valid path expected").to_string());

    HashSet::from_iter(iter)
}

fn find_tree(repo: &Repository, oid: Oid) -> Tree {
    repo.find_commit(oid)
        .expect("Could not find commit")
        .tree()
        .expect("Could not get tree")
}

fn find_base_oid(repo: &Repository, base: String) -> Oid {
    let oid = Oid::from_str(base.as_str());

    if oid.is_ok() {
        return oid.unwrap();
    }

    repo.find_branch(&base, BranchType::Local)
        .expect("Faild to fetch branch")
        .get()
        .target()
        .expect("Failed to fetch target")
}

fn committed_changes(repo: &Repository, base: String) -> HashSet<String> {
    let base_oid = find_base_oid(repo, base);

    let head_oid = repo
        .head()
        .expect("Failed to fetch HEAD")
        .target()
        .expect("Failed to fetch target");

    let base = repo
        .merge_base(base_oid, head_oid)
        .expect("Failed to find merge base");

    let old_tree = find_tree(repo, base);
    let new_tree = find_tree(repo, head_oid);

    let diffs = repo
        .diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)
        .expect("Could not get the diffs");

    let files = diffs.deltas().map(&map_new_files);

    HashSet::from_iter(files)
}

fn map_new_files(delta: DiffDelta) -> String {
    delta
        .new_file()
        .path()
        .expect("Could not get path")
        .to_str()
        .expect("Could not get path")
        .to_string()
}

pub fn changes(base: String) -> HashSet<String> {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let committed = committed_changes(&repo, base);
    let uncommitted = uncommitted_changes(&repo);

    committed.into_iter().chain(uncommitted).collect()
}
