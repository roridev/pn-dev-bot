use std::fmt::Display;

pub enum PullRequest {
    Open,
    Closed,
    Draft,
    Merged,
}

impl Display for PullRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            PullRequest::Open => "<:gh_pr_closed:1008308520669683732>",
            PullRequest::Closed => "<:gh_pr_closed:1008308520669683732>",
            PullRequest::Draft => "<:gh_pr_draft:1008308521814728756>",
            PullRequest::Merged => "<:gh_pr_merged:1008308523341451374>",
        };
        write!(f, "{}", result)
    }
}

pub enum Issue {
    Open,
    Closed,
    NotPlanned,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Issue::Open => "<:gh_issue_open:1008308515527471104>",
            Issue::Closed => "<:gh_issue_closed:1009103024330903573>",
            Issue::NotPlanned => "<:gh_issue_notplanned:1009103026511937598>",
        };
        write!(f, "{}", result)
    }
}

pub enum Action {
    Refresh,
    Copy,
    Updated,
    OpenLink,
    Error,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Action::Refresh => "<:gh_refresh:1008341077314515014>",
            Action::Copy => "<:gh_copy:1008341074508521594>",
            Action::Updated => "<:gh_updated:1008308527913259028>",
            Action::OpenLink => "<:gh_open_link:1008308518908084277>",
            Action::Error => "<:gh_error:1008308514302722200>",
        };
        write!(f, "{}", result)
    }
}

pub enum Misc {
    File,
    Tag,
    Lock,
    LeftArrow,
    Repository,
}

impl Display for Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Misc::File => "<:gh_file:1008341075980726312>",
            Misc::Tag => "<:gh_tag:1008308526281670666>",
            Misc::Lock => "<:gh_lock:1008308517763039273>",
            Misc::LeftArrow => "<:gh_left_arrow:1008308516794142720>",
            Misc::Repository => "<:gh_repo:1009103027757654096>",
        };
        write!(f, "{}", result)
    }
}
