use poise::serenity_prelude as serenity;

use crate::utils::formatting::Format;
use crate::utils::github::{get_issues_and_prs_with_labels, Queryable};
use crate::{Context, Error};

#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum WaitingTag {
    Review,
    Merge,
    InGameTest,
    Response,
}

impl Queryable for WaitingTag {
    fn to_query(&self) -> String {
        match self {
            WaitingTag::Review => "Status: Awaiting Review".to_string(),
            WaitingTag::Merge => "Status: Awaiting Merge".to_string(),
            WaitingTag::InGameTest => "Status: Awaiting In Game Testing".to_string(),
            WaitingTag::Response => "Status: Response".to_string(),
        }
    }
}

#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum Query {
    PullRequest,
    Issue,
    All,
}

#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum Sorting {
    Created,
    Updated,
    Comments,
}

impl Sorting {
    fn to_param(&self) -> octocrab::params::issues::Sort {
        match self {
            Sorting::Created => octocrab::params::issues::Sort::Created,
            Sorting::Updated => octocrab::params::issues::Sort::Updated,
            Sorting::Comments => octocrab::params::issues::Sort::Comments,
        }
    }
}

#[poise::command(slash_command)]
pub async fn waiting(
    ctx: Context<'_>,
    #[description = "The type of tag to be searched"] tag: WaitingTag,
    #[description = "Default: All"] kind: Option<Query>,
    #[description = "Default: Activity"] sort: Option<Sorting>,
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Command called on DMS");

    if guild.id.0 != ctx.data().ids.guild {
        return Ok(());
    }

    let member = guild.member(ctx.discord(), ctx.author().id).await?;

    if !member
        .roles
        .contains(&serenity::RoleId(ctx.data().ids.developer_role))
    {
        ctx.send(|reply| {
            reply
                .content("Unauthorized access. Developer-Only Command.")
                .ephemeral(true)
        })
        .await?;

        return Ok(());
    }

    let issues = get_issues_and_prs_with_labels(
        vec![tag],
        octocrab::params::State::Open,
        sort.unwrap_or(Sorting::Updated).to_param(),
        1,
    )
    .await?;

    let content: Vec<String> = issues.into_iter().map(|it| it.fmt()).collect();

    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(format!(
                    "Found {} issues for `Awaiting: {:?}` in {:?}",
                    content.len(),
                    tag,
                    kind.unwrap_or(Query::All)
                ))
                .description(content.join("\n"))
        })
    })
    .await?;

    Ok(())
}
