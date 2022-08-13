use crate::{Context, Error};
use octocrab::models::issues::Issue;

/// Gets the information about a issue
#[poise::command(slash_command)]
pub async fn issue(ctx: Context<'_>, #[description = "The Issue #"] id: u64) -> Result<(), Error> {
    let issue = search_github(id).await?;
    let mut description = issue.body.unwrap_or(String::from("No description."));
    description.truncate(250);
    let tags: Vec<String> = issue
        .labels
        .iter()
        .map(|it| format!("`{}`", it.name))
        .collect();
    let is_squash = tags.iter().any(|x| x.contains("Squash"));
    let assignees: Vec<String> = issue.assignees.iter().map(|it| it.login.clone()).collect();
    let url = issue.html_url.to_string();
    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(format!("{} (#{})", issue.title, issue.number))
                .footer(|footer| {
                    footer
                        .text(format!("Author: {}", issue.user.login))
                        .icon_url(issue.user.avatar_url.to_string())
                })
                .description(format!("➠ [Jump to Issue]({})\n{}", url, description))
                .field("Labels", tags.join(", "), false)
                .field(
                    "Assignees",
                    if assignees.is_empty() {
                        "No assignees.".to_string()
                    } else {
                        assignees.join(", ")
                    },
                    false,
                )
                .field(
                    "More Information",
                    format!(
                        "Last Updated: <t:{}:f>\n\
                       Created at: <t:{}:f>\n\
                       Recommended Merge Type: {}
                       ➠ [Jump to Issue]({})",
                        issue.updated_at.timestamp(),
                        issue.created_at.timestamp(),
                        if is_squash {
                            "**MERGING WITH SQUASH IS REQUIRED**"
                        } else {
                            "Normal"
                        },
                        url
                    ),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}

async fn search_github(id: u64) -> Result<Issue, octocrab::Error> {
    octocrab::instance()
        .issues("powernukkit", "powernukkit")
        .get(id)
        .await
}
