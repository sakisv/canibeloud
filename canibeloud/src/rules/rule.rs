use chrono::{Local, Timelike as _};
use chrono_tz::Tz;
use chrono_tz::EET;

pub struct RuleResponse {
    pub can_i_be_loud: bool,
    pub response_text: String,
    pub secondary_text: String,
}

pub trait Rulelike {
    fn can_i_be_loud(&self, timezone: String) -> RuleResponse;
}

pub struct OtherTimezone{}

impl Rulelike for OtherTimezone {
    fn can_i_be_loud(&self, timezone: String) -> RuleResponse {
        let mut r_response = RuleResponse {
            can_i_be_loud: true,
            response_text: String::from("Probably"),
            secondary_text: String::from("(But with reason)"),
        };

        // if we cannot parse the timezone, we default to EET (UTC+2) as it seems to be
        // the one with the most countries
        // source: https://en.wikipedia.org/wiki/List_of_UTC_offsets
        let other_tz: Tz = timezone.parse().unwrap_or(EET);
        let now = Local::now().with_timezone(&other_tz);
        let start = now.with_hour(23).unwrap().with_minute(0).unwrap();
        let end = now.with_hour(7).unwrap().with_minute(0).unwrap();
        if now >= start && now <= end {
            r_response = RuleResponse { can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from("Use common sense"), };
        }
        r_response
    }
}
