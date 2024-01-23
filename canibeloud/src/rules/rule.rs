pub struct RuleResponse {
    pub can_i_be_loud: bool,
    pub response_text: String,
    pub secondary_text: String,
}

pub trait Rulelike {
    fn can_i_be_loud() -> RuleResponse;
}
