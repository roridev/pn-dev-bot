use octocrab::models::issues::Issue;
use poise::serenity_prelude as serenity;

use crate::{Context, Error, utils::Formattable};

#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum WaitingTag {
    Review,
    Merge,
    Response,
    InGameTest,
}

impl WaitingTag {
    fn to_query(&self) -> &'static str {
        match &self {
            WaitingTag::Review => "label:\"Status: Awaiting Review\"",
            WaitingTag::Merge => "label:\"Status: Awaiting Merge\"",
            WaitingTag::Response => "label:\"Status: Awaiting Response\"",
            WaitingTag::InGameTest => "label:\"Status: Awaiting In Game Testing\"",
        }
    }
}

#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum Query {
    All,
    PullRequest,
    Issue,
}

impl Query {
    fn to_query(&self) -> &'static str {
        match &self {
            Query::All => "",
            Query::PullRequest => "is:pr",
            Query::Issue => "is:issue",
        }
    }
}

/// Searches Github for Issues/Pull Requests matching the `Waiting:` tags.
#[poise::command(slash_command)]
pub async fn waiting(
    ctx: Context<'_>,
    #[description = "The type of the tag to be matched"] tag: WaitingTag,
    #[description = "If query should filter only Pull Requests or Issues. Searches both if not specified."]
    kind: Option<Query>,
    #[description = "If the results should be sorted by recent activity instead of issue #"]
    activity: Option<bool>,
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Command called on DMs");
    if guild.id.0 == ctx.data().guild_id {
        let member = guild.member(ctx.discord(), ctx.author().id).await?;
        if !member
            .roles
            .contains(&serenity::RoleId(ctx.data().developer_role))
        {
            ctx.send(|reply| {
                reply
                    .content("Unauthorized: Developer-Only Command.")
                    .ephemeral(true)
            })
            .await?;
        } else {
            send_cool_embed(
                ctx,
                tag,
                kind.unwrap_or(Query::All),
                activity.unwrap_or(true),
            )
            .await?;
        }
    }
    Ok(())
}

async fn send_cool_embed(
    ctx: Context<'_>,
    tag: WaitingTag,
    query: Query,
    sort: bool,
) -> Result<(), Error> {
    let issues = search_github(tag, query, sort).await?;
    let text: Vec<String> = issues.iter().map(|k| k.format_it()).collect();
    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title(format!(
                    "Found {} issues for `Awaiting: {:?}` in {:?}",
                    issues.len(),
                    tag,
                    query
                ))
                .description(text.join("\n"))
        })
    })
    .await?;

    Ok(())
}

async fn search_github(
    tag: WaitingTag,
    query: Query,
    sort: bool,
) -> Result<Vec<Issue>, octocrab::Error> {
    let sort = if sort { "sort:updated-desc" } else { "" };
    let request = format!(
        "PowerNukkit PowerNukkit {} {} is:open {}",
        tag.to_query(),
        query.to_query(),
        sort
    );
    let results = octocrab::instance()
        .search()
        .issues_and_pull_requests(&request)
        .send()
        .await?;

    return Ok(results.items);
}
