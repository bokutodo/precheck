use anyhow::{Context, Result};
use git2::Repository;

/// Get the diff of staged changes
pub fn get_staged_diff() -> Result<String> {
    let repo = Repository::open(".")
        .context("Failed to open git repository. Are you in a git project?")?;

    let mut diff_options = git2::DiffOptions::new();
    
    // Get the diff between HEAD and the index (staged changes)
    let head = repo.head()?.peel_to_tree()?;
    let diff = repo.diff_tree_to_index(
        Some(&head),
        None,
        Some(&mut diff_options)
    )?;

    // Convert diff to a string
    let mut diff_text = String::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        let content = std::str::from_utf8(line.content()).unwrap_or("");
        diff_text.push_str(content);
        true
    })?;

    if diff_text.is_empty() {
        anyhow::bail!("No staged changes found. Use 'git add' to stage files first.");
    }

    Ok(diff_text)
}

/// Get list of staged files
pub fn get_staged_files() -> Result<Vec<String>> {
    let repo = Repository::open(".")
        .context("Failed to open git repository")?;

    let head = repo.head()?.peel_to_tree()?;
    let mut diff_options = git2::DiffOptions::new();
    let diff = repo.diff_tree_to_index(Some(&head), None, Some(&mut diff_options))?;

    let mut files = Vec::new();
    diff.foreach(
        &mut |delta, _progress| {
            if let Some(path) = delta.new_file().path() {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(files)
}