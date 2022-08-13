use octocrab::models::issues::Issue;
use octocrab::models::Label;

pub fn format_it(issue: &Issue) -> String {
    let size: Vec<&Label> = issue
        .labels
        .iter()
        .filter(|it| it.name.contains("size/"))
        .collect();

    let base = format!(
        "**#{}** - [{}]({})",
        issue.number,
        issue.title,
        issue.html_url.to_string()
    );

    if !size.is_empty() {
        format!(" {} ➠ `{}`", base, size.first().unwrap().name)
    } else {
        base
    }
}
