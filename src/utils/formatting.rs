use octocrab::models::issues::Issue;
use octocrab::models::pulls::PullRequest;

use super::emoji;

pub trait Format {
    fn fmt(&self) -> String;
}

impl Format for Issue {
    fn fmt(&self) -> String {
        let status = get_state_issue(self);

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
                    || it.name.starts_with("Type:")
                    || it.name.contains("priority")
            })
            .map(|it| format!("`{}`", it.name))
            .collect();

        let base = format!(
            "{} **#{}** [{}]({}) · {} {}",
            status,
            number,
            emoji::Action::OpenLink,
            self.html_url,
            self.title,
            locked
        );

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

fn get_state_issue(issue: &Issue) -> String {
    if issue.pull_request.is_some() {
        return if issue.state == "closed" {
            emoji::PullRequest::Closed.to_string()
        } else {
            emoji::PullRequest::Open.to_string()
        };
    } else {
        return if issue.state == "closed" {
            emoji::Issue::Closed.to_string()
        } else {
            emoji::Issue::Open.to_string()
        };
    }
}

impl Format for PullRequest {
    fn fmt(&self) -> String {
        let status = get_state(&self);
        let locked = if self.locked {
            format!("{}", emoji::Misc::Lock)
        } else {
            "".to_string()
        };
        let number = &self.number;
        let merge = self.mergeable.unwrap_or(true);

        let base = format!(
            "{} {} **#{}** · {} {}",
            if merge {
                "".to_string()
            } else {
                emoji::Action::Error.to_string()
            },
            status,
            number,
            self.clone().title.unwrap_or("Unknown Title".to_string()),
            locked
        );

        base
    }
}

fn get_state(pr: &PullRequest) -> emoji::PullRequest {
    if pr.merged_at.is_some() {
        return emoji::PullRequest::Merged;
    }

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
