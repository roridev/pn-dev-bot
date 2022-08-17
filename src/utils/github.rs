use octocrab::models::issues::Issue;
use octocrab::models::pulls::PullRequest;
use octocrab::Page;

pub trait Queryable {
    fn to_query(&self) -> String;
}

impl Queryable for String {
    fn to_query(&self) -> String {
        (*self).clone()
    }
}

impl Queryable for &str {
    fn to_query(&self) -> String {
        self.to_string()
    }
}

pub async fn get_issues_and_prs_with_labels<T: Queryable>(
    labels: Vec<T>,
    state: octocrab::params::State,
    sort: octocrab::params::issues::Sort,
    page: u32,
) -> Result<Page<Issue>, octocrab::Error> {
    let inner: Vec<String> = labels.iter().map(|it| it.to_query()).collect();
    octocrab::instance()
        .issues("PowerNukkit", "PowerNukkit")
        .list()
        .state(state)
        .sort(sort)
        .labels(&inner)
        .page(page)
        .per_page(100)
        .send()
        .await
}

pub async fn get_pr(id: u64) -> Result<PullRequest, octocrab::Error> {
    octocrab::instance()
        .pulls("PowerNukkit", "PowerNukkit")
        .get(id)
        .await
}

pub async fn get_issue(id: u64) -> Result<Issue, octocrab::Error> {
    octocrab::instance()
        .issues("PowerNukkit", "PowerNukkit")
        .get(id)
        .await
}

pub fn is_pull_request(issue: &Issue) -> bool {
    issue.pull_request.is_some()
}

pub mod labels {
}
