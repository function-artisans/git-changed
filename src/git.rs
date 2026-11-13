use anyhow::Context;
use anyhow::Result;
use git2::DiffDelta;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use git2::Tree;
use std::collections::BTreeSet;
use std::env;

const CONFIG_PATHS: [&str; 2] = ["changed.defaultBranch", "init.defaultBranch"];
const DEFAULT_BRANCH: &str = "main";

fn status_options() -> StatusOptions {
    let mut options = StatusOptions::default();

    options.include_untracked(true);
    options.recurse_untracked_dirs(true);
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

fn find_base_oid(repo: &Repository, base: &str) -> Result<Oid> {
    let commit = repo
        .revparse_single(base)
        .and_then(|object| object.peel_to_commit())
        .with_context(|| format!("Cannot find the commit of {base}"))?;

    Ok(commit.id())
}

fn committed_changes(repo: &Repository, base: &str) -> Result<Vec<String>> {
    let base_oid = find_base_oid(repo, base)?;

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

fn default_branch(repo: &Repository) -> Result<String> {
    let config = repo.config().context("Cannot read the git config")?;

    let branch = CONFIG_PATHS
        .into_iter()
        .find_map(|path| config.get_string(path).ok());

    Ok(branch.unwrap_or_else(|| DEFAULT_BRANCH.to_string()))
}

pub fn open_repository() -> Result<Repository> {
    let cwd = env::current_dir()?;

    Repository::discover(&cwd)
        .with_context(|| format!("Cannot find a repository at {}", cwd.display()))
}

pub fn changes(repo: &Repository, base: Option<&str>) -> Result<BTreeSet<String>> {
    let base = match base {
        Some(base) => base.to_string(),
        None => default_branch(repo)?,
    };

    let committed = committed_changes(repo, &base)?.into_iter();
    let uncommitted = uncommitted_changes(repo)?.into_iter();

    let files = committed.chain(uncommitted);

    Ok(BTreeSet::from_iter(files))
}
