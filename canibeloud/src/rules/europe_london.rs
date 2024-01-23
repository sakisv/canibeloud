// 11pm to 7am
// source: https://www.gov.uk/guidance/noise-nuisances-how-councils-deal-with-complaints

use chrono::{Local, Timelike as _};
use super::rule::{Rulelike, RuleResponse};
use chrono_tz::Europe::London;

pub struct EuropeLondon {}

impl Rulelike for EuropeLondon {
    fn can_i_be_loud() -> RuleResponse {
        let mut r_response = RuleResponse {
            can_i_be_loud: true,
            response_text: String::from("Yes"),
            secondary_text: String::from("(But with reason)"),
        };
        let now = Local::now().with_timezone(&London);
        let start = now.with_hour(23).unwrap().with_minute(0).unwrap();
        let end = now.with_hour(7).unwrap().with_minute(0).unwrap();
        if now >= start && now <= end {
            r_response = RuleResponse { can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from(""), };
        }
        r_response
    }
}
