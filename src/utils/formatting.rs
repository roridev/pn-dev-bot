use octocrab::models::issues::Issue;
use octocrab::models::pulls::PullRequest;

use super::emoji;

pub trait Format {
    fn fmt(&self) -> String;
}

impl Format for Issue {
    fn fmt(&self) -> String {
        let status = if self.state == "closed" {
            emoji::Issue::Closed
        } else {
            emoji::Issue::Open
        };

        let locked = if self.locked {
            emoji::Misc::Lock.to_string()
        } else {
            "".to_string()
        };
        let number = &self.number;

        let filtered_issues: Vec<String> = self
            .labels
            .iter()
            .filter(|it| {
                it.name.contains("Resolution")
                    || it.name.contains("size")
                    || it.name.contains("Type")
                    || it.name.contains("priority")
            })
            .map(|it| format!("`{}`", it.name))
            .collect();

        let base = format!("{} **#{}** {} {}", status, number, self.title, locked);

        if !filtered_issues.is_empty() {
            return format!(
                "{}\n{} {}",
                base,
                emoji::Misc::Tag,
                filtered_issues.join(" ")
            );
        } else {
            return base;
        }
    }
}

fn get_state(pr: &PullRequest) -> emoji::PullRequest {
    if pr.draft.unwrap_or(false) {
        emoji::PullRequest::Draft
    } else {
        if pr
            .state
            .clone()
            .unwrap_or(octocrab::models::IssueState::Open)
            == octocrab::models::IssueState::Open
        {
            emoji::PullRequest::Open
        } else {
            emoji::PullRequest::Closed
        }
    }
}
