// 11pm to 7am
// source: https://www.gov.uk/guidance/noise-nuisances-how-councils-deal-with-complaints

use chrono::{DateTime, Utc, Timelike as _};
use super::rule::{Rulelike, RuleResponse};
use chrono_tz::Tz;

pub struct EuropeLondon {}

impl Rulelike for EuropeLondon {
    fn can_i_be_loud(&self, utc_now: DateTime<Utc>, _: String) -> RuleResponse {
        let london_tz: Tz = "Europe/London".parse().unwrap();
        let now = utc_now.with_timezone(&london_tz);

        let mut r_response = RuleResponse {
            can_i_be_loud: true,
            response_text: String::from("Yes"),
            secondary_text: String::from("(But within reason)"),
            tz_datetime: format!("{}", now.format("%A %d %B @ %H:%M")),
            tz_found: true,
            source_url: "https://www.gov.uk/guidance/noise-nuisances-how-councils-deal-with-complaints".to_owned(),
        };

        let start = now.with_hour(23).unwrap().with_minute(0).unwrap();
        let end = now.with_hour(7).unwrap().with_minute(0).unwrap();
        if now >= start || now <= end {
            r_response = RuleResponse { can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from(""), ..r_response};
        }
        r_response
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};
    use chrono_tz::Europe::London;

    use crate::rules::{europe_london::EuropeLondon, rule::Rulelike};

    #[test]
    fn test_europe_london_rule() {
        let europe_london = EuropeLondon{};
        let tz = "Europe/London".to_owned();

        let morning = London.with_ymd_and_hms(2023, 11, 13, 08, 15, 00).unwrap();
        let morning_res = europe_london.can_i_be_loud(morning.with_timezone(&Utc), tz.clone());
        assert_eq!(true, morning_res.can_i_be_loud);

        let afternoon = London.with_ymd_and_hms(2023, 11, 13, 19, 15, 00).unwrap();
        let afternoon_res = europe_london.can_i_be_loud(afternoon.with_timezone(&Utc), tz.clone());
        assert_eq!(true, afternoon_res.can_i_be_loud);

        let night = London.with_ymd_and_hms(2023, 11, 13, 23, 15, 00).unwrap();
        let night_res = europe_london.can_i_be_loud(night.with_timezone(&Utc), tz.clone());
        assert_eq!(false, night_res.can_i_be_loud);
    }
}
