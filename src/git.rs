use anyhow::Context;
use anyhow::Result;
use git2::DiffDelta;
use git2::ErrorCode;
use git2::Oid;
use git2::Repository;
use git2::StatusOptions;
use git2::Tree;
use std::collections::BTreeSet;
use std::env;

const CONFIG_PATHS: [&str; 2] = ["changed.defaultBranch", "init.defaultBranch"];
const DEFAULT_BRANCH: &str = "main";

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

fn default_branch(repo: &Repository) -> Result<String> {
    let config = repo.config().context("Cannot read the git config")?;

    for path in CONFIG_PATHS {
        match config.get_string(path) {
            Ok(branch) => return Ok(branch),
            Err(error) if error.code() == ErrorCode::NotFound => continue,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("Cannot read {path} from the git config"))
            }
        }
    }

    Ok(DEFAULT_BRANCH.to_string())
}

fn committed_changes(repo: &Repository, base: &str) -> Result<Vec<String>> {
    let base_oid = find_commit_oid(repo, base)?;
    let head_oid = find_commit_oid(repo, "HEAD")?;

    let merge_base_oid = repo
        .merge_base(base_oid, head_oid)
        .with_context(|| format!("Cannot find a merge base of {base} and HEAD"))?;

    let old_tree = find_tree(repo, merge_base_oid)?;
    let new_tree = find_tree(repo, head_oid)?;

    let diff = repo
        .diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)
        .with_context(|| format!("Cannot compare {base} with HEAD"))?;

    diff.deltas().map(changed_path).collect()
}

fn find_commit_oid(repo: &Repository, revision: &str) -> Result<Oid> {
    let commit = repo
        .revparse_single(revision)
        .and_then(|object| object.peel_to_commit())
        .with_context(|| format!("Cannot find the commit of {revision}"))?;

    Ok(commit.id())
}

fn find_tree(repo: &Repository, oid: Oid) -> Result<Tree<'_>> {
    repo.find_commit(oid)
        .and_then(|commit| commit.tree())
        .with_context(|| format!("Cannot find the tree of {oid}"))
}

fn changed_path(delta: DiffDelta) -> Result<String> {
    let path = delta
        .new_file()
        .path()
        .context("A changed file has no path")?;

    let text = path
        .to_str()
        .with_context(|| format!("The path {} is not UTF-8", path.display()))?;

    Ok(text.to_string())
}

fn uncommitted_changes(repo: &Repository) -> Result<Vec<String>> {
    let statuses = repo
        .statuses(Some(&mut status_options()))
        .context("Cannot read the status of the working directory")?;

    statuses
        .iter()
        .map(|entry| {
            let path = entry
                .path()
                .context("A changed file has a path that is not UTF-8")?;

            Ok(path.to_string())
        })
        .collect()
}

fn status_options() -> StatusOptions {
    let mut options = StatusOptions::default();

    options.include_untracked(true);
    options.recurse_untracked_dirs(true);
    options.exclude_submodules(true);

    options
}
