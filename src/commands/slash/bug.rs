use octocrab::models::issues::Issue;

use crate::commands::slash::waiting::Sorting;
use crate::utils::emoji;
use crate::utils::formatting::Format;
use crate::utils::github::labels::{ConfirmationStatus, Priority};
use crate::utils::github::{get_issues_and_prs_with_labels, is_pull_request, Queryable};
use crate::utils::vec::stringify_vec;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn bug(
    ctx: Context<'_>,
    status: ConfirmationStatus,
    prio: Option<i64>,
    sort: Option<Sorting>,
) -> Result<(), Error> {
    let mut tags = vec![status.to_query(), "Type: bug".to_query()];

    if prio.is_some() {
        tags.insert(0, Priority::Priority(prio.unwrap()).to_query())
    }

    let query = get_issues_and_prs_with_labels(
        tags,
        octocrab::params::State::Open,
        sort.unwrap_or(Sorting::Updated).to_param(),
        1,
    )
    .await?;

    let prs: Vec<String> = query
        .clone()
        .into_iter()
        .filter(|it| is_pull_request(&it))
        .map(|it| it.fmt())
        .collect();

    let issues: Vec<String> = query
        .into_iter()
        .filter(|it| !is_pull_request(&it))
        .map(|it| it.fmt())
        .collect();

    let title = format!(
        "Found `{}` {:?} Bugs {} | {}{} {}{}",
        prs.len() + issues.len(),
        status,
        if prio.is_some() {
            format!("for `prio={}`", prio.unwrap())
        } else {
            "".to_string()
        },
        emoji::Issue::Open,
        issues.len(),
        emoji::PullRequest::Open,
        prs.len()
    );

    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(title)
                .field(
                    format!("{} Issues", emoji::Issue::Open),
                    stringify_vec(&issues, "None.", "\n"),
                    false,
                )
                .field(
                    format!("{} Pull Requests", emoji::PullRequest::Open),
                    stringify_vec(&prs, "None.", "\n"),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}
