use std::fmt::Display;

use color_eyre::eyre::Result;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Editor, FuzzySelect, Input};
use git2::{Commit, Repository, RepositoryOpenFlags, StatusOptions};

use crate::issue::Issue;

pub const MAX_SHORT_DESC: usize = 50;

pub mod issue;

#[derive(Debug, Clone, Copy)]
pub enum CommitType {
    Feature,
    Fix,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Chore,
}
impl Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Feature => "feat: A new feature",
                Self::Fix => "fix: A bug fix",
                Self::Docs => "docs: Documentation only changes",
                Self::Style => "style: Changes that do not affect the meaning of the code",
                Self::Refactor =>
                    "refactor: A code change that neither fixes a bug nor adds a feature",
                Self::Perf => "perf: A code change that improves performance",
                Self::Test => "test: Adding missing or correcting existing tests",
                Self::Chore => "chore: Changes that don't modify src files",
            }
        )
    }
}

fn main() -> Result<()> {
    let repo = Repository::open_ext(
        std::env::current_dir()?,
        RepositoryOpenFlags::empty(),
        &[] as &[&std::ffi::OsStr],
    )?;
    let user_signature = repo.signature()?;

    let statuses = repo.statuses(Some(StatusOptions::new().show(git2::StatusShow::Index)))?;

    let is_staged_files = statuses
        .iter()
        .map(|e| e.head_to_index())
        .any(|x| x.is_some());

    if !is_staged_files {
        println!("Empty worktree");
        return Ok(());
    }

    let theme = ColorfulTheme {
        prompt_style: Style::new().white(),
        defaults_style: Style::new().dim(),
        active_item_style: Style::new().yellow().italic(),
        inactive_item_style: Style::new().black(),
        ..Default::default()
    };

    // Get data
    let items = &[
        CommitType::Feature,
        CommitType::Fix,
        CommitType::Docs,
        CommitType::Style,
        CommitType::Refactor,
        CommitType::Perf,
        CommitType::Test,
        CommitType::Chore,
    ];

    let commit_type = items[FuzzySelect::with_theme(&theme)
        .with_prompt("Select type of change you're committing:")
        .items(items)
        .interact()?];

    let scope: String = Input::with_theme(&theme)
        .with_prompt("What the scope of change:")
        .interact_text()?;

    let short_desc: String = Input::with_theme(&theme)
        .with_prompt(format!("Short desc (max {MAX_SHORT_DESC} chars)"))
        .validate_with(|x: &String| {
            if x.len() <= MAX_SHORT_DESC {
                Ok(())
            } else {
                Err("This message is too long")
            }
        })
        .interact_text()?;

    let long_desc: String = Input::with_theme(&theme)
        .with_prompt("Provide long description of the change: (empty to skip)")
        .allow_empty(true)
        .interact_text()?;

    let breaking_changes: bool = Confirm::with_theme(&theme)
        .default(false)
        .with_prompt("Are there any breaking changes?")
        .interact()?;

    let affected_issue: bool = Confirm::with_theme(&theme)
        .default(false)
        .with_prompt("Does this change affect any issue?")
        .interact()?;

    let issue = if affected_issue {
        Issue::new(&theme)?.to_string()
    } else {
        "".to_string()
    };

    // Format commit message
    let message = format!(
        "{}({scope}): {short_desc}\n\n{long_desc}\n\n{issue}\n{}",
        // Commit type
        commit_type.to_string().splitn(2, ':').collect::<Vec<_>>()[0],
        // Breaking changes
        if breaking_changes {
            let message = Editor::new().edit("Breaking change description")?.unwrap();
            format!("BREAKING CHANGE: {message}")
        } else {
            "".to_string()
        }
    );

    println!("{message}");

    let mut index = repo.index()?;
    let tree = repo.find_tree(index.write_tree()?)?;

    let parent = repo.head().map(|x| x.target().map(|x| repo.find_commit(x)));

    let parent_full: &[&Commit] = if let Ok(Some(commit)) = parent {
        &[&commit?]
    } else {
        &[]
    };

    let commit_id = repo.commit(
        Some("HEAD"),
        &user_signature,
        &user_signature,
        &message,
        &tree,
        parent_full,
    )?;
    println!("Commit id: {commit_id}");

    Ok(())
}
