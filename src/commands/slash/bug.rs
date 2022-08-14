use crate::{Context, Error, utils::Formattable};
use octocrab::models::issues::Issue;

#[derive(Debug, Copy, Clone)]
enum BugStatus {
    Confirmed,
    Unconfirmed,
    Uncategorized,
}

impl BugStatus {
    fn to_request(&self) -> &'static str {
        match self {
            BugStatus::Confirmed => "label:\"Status: Confirmed\"",
            BugStatus::Unconfirmed => "label:\"Status: Unconfirmed\"",
            BugStatus::Uncategorized => {
                "-label:\"Status: Confirmed\" -label:\"Status: Unconfirmed\""
            }
        }
    }
}
/// Searches Github for bugs
#[poise::command(
    slash_command,
    subcommands("confirmed", "unconfirmed", "uncategorized")
)]
pub async fn bug(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Returns all confirmed bugs
#[poise::command(slash_command)]
pub async fn confirmed(
    context: Context<'_>,
    #[description = "If the results should be sorted by recent activity instead of issue #"]
    activity: Option<bool>,
) -> Result<(), Error> {
    let bugs = get_bugs(BugStatus::Confirmed, activity.unwrap_or(true)).await?;
    let text: Vec<String> = bugs.iter().map(|x| x.format_it()).collect();

    context
        .send(|response| {
            response.embed(|embed| {
                embed
                    .title(format!("Found {} confirmed bugs", bugs.len()))
                    .description(text.join("\n"))
            })
        })
        .await?;

    Ok(())
}

/// Returns all unconfirmed bugs
#[poise::command(slash_command)]
pub async fn unconfirmed(
    context: Context<'_>,
    #[description = "If the results should be sorted by recent activity instead of issue #"]
    activity: Option<bool>,
) -> Result<(), Error> {
    let bugs = get_bugs(BugStatus::Unconfirmed, activity.unwrap_or(true)).await?;
    let text: Vec<String> = bugs.iter().map(|x| x.format_it()).collect();

    context
        .send(|response| {
            response.embed(|embed| {
                embed
                    .title(format!("Found {} unconfirmed bugs", bugs.len()))
                    .description(text.join("\n"))
            })
        })
        .await?;

    Ok(())
}

/// Returns all uncategorized bugs
#[poise::command(slash_command)]
pub async fn uncategorized(
    context: Context<'_>,
    #[description = "If the results should be sorted by recent activity instead of issue #"]
    activity: Option<bool>,
) -> Result<(), Error> {
    let bugs = get_bugs(BugStatus::Uncategorized, activity.unwrap_or(true)).await?;
    let text: Vec<String> = bugs.iter().map(|x| format!("#{}", x.number)).collect();

    context.send(|response| {
        response.embed(|embed| {
            embed.title(format!("Found {} uncategorized bugs", bugs.len()))
                .description(format!("Here are all the bugs that weren't yet categorized.\n\
                Since this list is usually **big**, details about individual issues will be omitted.\n\
                Please use `/issue` to get information about a given id of your choice.\n\n\
                {}",text.join(", ")))
        })
    }).await?;

    Ok(())
}

async fn get_bugs(status: BugStatus, sort: bool) -> Result<Vec<Issue>, octocrab::Error> {
    let sorting = if sort { "sort:updated-desc" } else { "" };
    let request = format!(
        "PowerNukkit PowerNukkit {} is:issue is:open label:\"Type: bug\" {}",
        status.to_request(),
        sorting
    );

    let mut items: Vec<Issue> = vec![];

    let mut response = octocrab::instance()
        .search()
        .issues_and_pull_requests(&request)
        .send()
        .await?;
    let pages = response.number_of_pages().unwrap_or(1);
    items.append(&mut response.items);
    if pages > 1 {
        for i in 2..(pages + 1) {
            response = octocrab::instance()
                .search()
                .issues_and_pull_requests(&request)
                .page(i)
                .send()
                .await?;
            items.append(&mut response.items);
        }
    }

    return Ok(items);
}
