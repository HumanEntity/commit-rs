use std::fmt::Display;

use color_eyre::eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, MultiSelect, Select};

pub const MAX_SHORT_DESC: usize = 50;

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
    println!("Hello, world!");
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

    let commit_type = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select type of change you're committing:")
        .items(items)
        .interact()?;

    let scope: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What the scope of change:")
        .interact_text()?;

    let short_desc: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Short desc (max {MAX_SHORT_DESC} chars)"))
        .validate_with(|x: &String| {
            if x.len() <= MAX_SHORT_DESC {
                Ok(())
            } else {
                Err("This message is too long")
            }
        })
        .interact_text()?;

    let long_desc: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide long description of the change: (empty to skip)")
        .allow_empty(true)
        .interact_text()?;

    let breaking_changes: bool = Confirm::with_theme(&ColorfulTheme::default())
        .default(false)
        .with_prompt("Are there any breaking changes?")
        .interact()?;

    let affected_issue: bool = Confirm::with_theme(&ColorfulTheme::default())
        .default(false)
        .with_prompt("Does this change affect any issue?")
        .interact()?;

    Ok(())
}
