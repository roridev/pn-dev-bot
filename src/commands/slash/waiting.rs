
#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum WaitingTag {
    Review,
    Merge,
    InGameTest,
    Response,
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
