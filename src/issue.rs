use std::fmt::Display;

use color_eyre::eyre::Result;
use dialoguer::{theme::Theme, Confirm, FuzzySelect, Input};

#[derive(Debug, Clone, Copy)]
pub enum IssueKeyword {
    Close,
    Closes,
    Closed,
    Fix,
    Fixes,
    Fixed,
    Resolve,
    Resolves,
    Resolved,
}

impl Display for IssueKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Close => "close",
                Self::Closes => "closes",
                Self::Closed => "closed",
                Self::Fix => "fix",
                Self::Fixes => "fixes",
                Self::Fixed => "fixed",
                Self::Resolve => "resolve",
                Self::Resolves => "Resolves",
                Self::Resolved => "Resolved",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum LinkedIssue {
    SameRepo {
        keyword: IssueKeyword,
        number: usize,
    },
    DifferentRepo {
        keyword: IssueKeyword,
        number: usize,
        owner: String,
        repo: String,
    },
}

impl LinkedIssue {
    pub fn new(theme: &dyn Theme) -> Result<Self> {
        let items = &[
            IssueKeyword::Close,
            IssueKeyword::Closes,
            IssueKeyword::Closed,
            IssueKeyword::Fix,
            IssueKeyword::Fixes,
            IssueKeyword::Fixed,
            IssueKeyword::Resolve,
            IssueKeyword::Resolves,
            IssueKeyword::Resolved,
        ];

        let keyword = items[FuzzySelect::with_theme(theme)
            .with_prompt("Choose adequate keyword:")
            .items(items)
            .interact()?];

        let (owner, repo): (String, String) = (
            Input::with_theme(theme)
                .with_prompt("Whats the owner of the repo (leave empty for current repo):")
                .allow_empty(true)
                .interact_text()?,
            Input::with_theme(theme)
                .with_prompt("What's the repo name (leave empty for current repo):")
                .allow_empty(true)
                .interact_text()?,
        );

        let number: usize = Input::with_theme(theme)
            .with_prompt("Enter issue number:")
            .interact_text()?;

        if owner.is_empty() && repo.is_empty() {
            Ok(Self::SameRepo { keyword, number })
        } else if !owner.is_empty() && !repo.is_empty() {
            Ok(Self::DifferentRepo {
                keyword,
                number,
                owner,
                repo,
            })
        } else {
            todo!()
        }
    }
}

impl Display for LinkedIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SameRepo { keyword, number } => {
                    format!("{keyword} #{number}")
                }
                Self::DifferentRepo {
                    keyword,
                    number,
                    owner,
                    repo,
                } => {
                    format!("{keyword} {owner}/{repo}#{number}")
                }
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Issue {
    Single(LinkedIssue),
    Multi(Vec<LinkedIssue>),
}

impl Issue {
    pub fn new(theme: &dyn Theme) -> Result<Self> {
        let is_multi = Confirm::with_theme(theme)
            .default(false)
            .with_prompt("Does this resolve multiple issues")
            .interact()?;

        if is_multi {
            Self::multi(theme)
        } else {
            Self::single(theme)
        }
    }

    fn single(theme: &dyn Theme) -> Result<Self> {
        Ok(Self::Single(LinkedIssue::new(theme)?))
    }

    fn multi(theme: &dyn Theme) -> Result<Self> {
        todo!()
    }
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(issue) => write!(f, "{issue}"),
            Self::Multi(issues) => todo!(),
        }
    }
}
