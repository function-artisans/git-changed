use anyhow::Context;
use anyhow::Result;
use git2::BranchType;
use git2::DiffDelta;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use git2::Tree;
use std::collections::HashSet;
use std::env;

fn status_options() -> StatusOptions {
    let mut options = StatusOptions::default();

    options.include_untracked(true);
    options.exclude_submodules(true);

    options
}

fn uncommitted_changes(repo: &Repository) -> Result<Vec<String>> {
    let mut options = status_options();

    let statuses = repo
        .statuses(Some(&mut options))
        .context("Could not get statuses")?;

    statuses
        .iter()
        .map(|entry| Ok(entry.path().context("Could not get path")?.to_string()))
        .collect()
}

fn find_tree(repo: &Repository, oid: Oid) -> Result<Tree<'_>> {
    repo.find_commit(oid)?
        .tree()
        .with_context(|| format!("Failed to find tree for {}", oid))
}

fn find_base_oid(repo: &Repository, base: String) -> Result<Oid> {
    let oid = Oid::from_str(base.as_str());

    if let Ok(value) = oid {
        return Ok(value);
    }

    repo.find_branch(&base, BranchType::Local)?
        .get()
        .target()
        .with_context(|| format!("Failed to fetch branch {}", base))
}

fn committed_changes(repo: &Repository, base: &String) -> Result<Vec<String>> {
    let base_oid = find_base_oid(repo, base.to_string())?;

    let head_oid = repo.head()?.target().context("Could not get oid")?;

    let base = repo
        .merge_base(base_oid, head_oid)
        .with_context(|| format!("Failed to find merge base for {} and HEAD", base))?;

    let old_tree = find_tree(repo, base)?;
    let new_tree = find_tree(repo, head_oid)?;

    let diffs = repo
        .diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)
        .with_context(|| "Could not get the diffs")?;

    diffs.deltas().map(&map_new_files).collect()
}

fn map_new_files(delta: DiffDelta) -> Result<String> {
    let path = delta
        .new_file()
        .path()
        .context("Failed to get file")?
        .to_str()
        .context("Failed to get path")?
        .to_string();

    Ok(path)
}

pub fn open_repository() -> Result<Repository> {
    let cwd = env::current_dir()?;

    Repository::discover(&cwd)
        .with_context(|| format!("Cannot find a repository at {}", cwd.display()))
}

pub fn changes(repo: &Repository, base: &String) -> Result<HashSet<String>> {
    let committed = committed_changes(repo, base)?.into_iter();
    let uncommitted = uncommitted_changes(repo)?.into_iter();

    let files = committed.chain(uncommitted);

    Ok(HashSet::from_iter(files))
}
