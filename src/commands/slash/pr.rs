use crate::utils::emoji;
use crate::utils::formatting::Format;
use crate::utils::github::get_pr;
use crate::utils::vec::stringify_vec;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn pr(
    ctx: Context<'_>,
    #[description = "The Pull Request number"] id: u64,
) -> Result<(), Error> {
    let pr = get_pr(id).await?;
    let title = pr.fmt();
    let number = pr.number;

    let labels: Vec<String> = pr
        .labels
        .unwrap_or(vec![])
        .iter()
        .map(|it| format!("`{}`", it.name))
        .collect();

    let conflict = !pr.mergeable.unwrap_or(true);
    let editable = pr.maintainer_can_modify;
    let last_update = pr.updated_at.expect("Unknown last update").timestamp();
    let link = pr.html_url.expect("Unknown PR url");

    let into = pr.base.ref_field;
    let from = pr.head.label.unwrap_or(pr.head.ref_field);
    let from_user = pr.user.expect("Unknown author");

    let requested_reviewers: Vec<String> = pr
        .requested_reviewers
        .unwrap_or(vec![])
        .iter()
        .map(|it| format!("[{}]({})", it.login, it.html_url))
        .collect();

    let description = format!(
        "`{}` {} `{}` {}\n{}<t:{}:f>\nOpen in Github: [{}]({})\n{}\n{}\n{} Tip: Use /git `{}` to \
         get git commands for this PR.",
        into,
        emoji::Misc::LeftArrow,
        from,
        if conflict {
            emoji::Action::Error.to_string()
        } else {
            "".to_string()
        },
        emoji::Action::Updated,
        last_update,
        emoji::Action::OpenLink,
        link,
        if !editable {
            format!(
                "{} Read-only. Maintainers can't edit branch directly.",
                emoji::Misc::Lock
            )
        } else {
            "".to_string()
        },
        if conflict {
            format!("{} **Merge conflict detected!**", emoji::Action::Error)
        } else {
            "".to_string()
        },
        emoji::Action::Copy,
        number
    );

    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(title)
                .description(description)
                .footer(|footer| {
                    footer
                        .icon_url(from_user.avatar_url)
                        .text(format!("Author: {}", from_user.login))
                })
                .field(
                    format!("{} Labels", emoji::Misc::Tag),
                    stringify_vec(&labels, "None.", " "),
                    false,
                )
                .field(
                    "Requested Reviewers",
                    stringify_vec(&requested_reviewers, "None.", ", "),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}
