use crate::utils::emoji;
use crate::utils::formatting::get_state_issue;
use crate::utils::github::get_issue;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn issue(
    ctx: Context<'_>,
    #[description = "The number of the issue"] number: u64,
) -> Result<(), Error> {
    let issue = get_issue(number).await?;

    let state = get_state_issue(&issue);

    let labels: Vec<String> = issue
        .labels
        .iter()
        .map(|it| format!("`{}`", it.name))
        .collect();

    let assignees: Vec<String> = issue
        .assignees
        .iter()
        .map(|it| format!("[{}]({})", it.login, it.html_url))
        .collect();

    let user = issue.user;
    let last_updated = issue.updated_at.timestamp();

    let description = format!(
        "{}<t:{}:f>\nOpen in Github: [{}]({})",
        emoji::Action::Updated,
        last_updated,
        emoji::Action::OpenLink,
        issue.html_url
    );

    let title = format!("{} **#{}** · {}", state, number, issue.title);

    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(title)
                .description(description)
                .footer(|footer| {
                    footer
                        .icon_url(user.avatar_url)
                        .text(format!("Author: {}", user.login))
                })
                .field(
                    "Assignees",
                    if assignees.is_empty() {
                        "None.".to_string()
                    } else {
                        assignees.join(", ")
                    },
                    false,
                )
                .field(
                    format!("{} Labels", emoji::Misc::Tag),
                    if labels.is_empty() {
                        "None".to_string()
                    } else {
                        labels.join(" ")
                    },
                    false,
                )
        })
    })
    .await?;

    Ok(())
}
