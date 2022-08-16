use octocrab::models::issues::Issue;
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

async fn get_issues_and_prs_with_labels<T: Queryable>(
    labels: Vec<T>,
    state: octocrab::params::State,
    page: u32,
) -> Result<Page<Issue>, octocrab::Error> {
    let inner: Vec<String> = labels.iter().map(|it| it.to_query()).collect();
    octocrab::instance()
        .issues("PowerNukkit", "PowerNukkit")
        .list()
        .state(state)
        .labels(&inner)
        .page(page)
        .per_page(100)
        .send()
        .await
}
