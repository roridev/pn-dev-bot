
#[derive(poise::ChoiceParameter, Copy, Clone, Debug)]
pub enum WaitingTag {
    Review,
    Merge,
    InGameTest,
    Response,
}
