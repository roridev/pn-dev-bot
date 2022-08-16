use crate::utils::formatting::Format;

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
