use chrono::{DateTime, Utc, Timelike as _};
use chrono_tz::Tz;
use chrono_tz::EET;

pub struct RuleResponse {
    pub can_i_be_loud: bool,
    pub response_text: String,
    pub secondary_text: String,
    pub tz_datetime: String,
    pub tz_found: bool,
    pub source_url: String,
}

pub trait Rulelike {
    fn can_i_be_loud(&self, utc_now: DateTime<Utc>, timezone: String) -> RuleResponse;
}

pub struct OtherTimezone{}

impl Rulelike for OtherTimezone {
    fn can_i_be_loud(&self, utc_now: DateTime<Utc>, timezone: String) -> RuleResponse {
        // if we cannot parse the timezone, we default to EET (UTC+2) as it seems to be
        // the one with the most countries
        // source: https://en.wikipedia.org/wiki/List_of_UTC_offsets
        let other_tz: Tz = timezone.parse().unwrap_or(EET);
        let now = utc_now.with_timezone(&other_tz);

        let mut r_response = RuleResponse {
            can_i_be_loud: true,
            response_text: String::from("Probably"),
            secondary_text: String::from("(But within reason)"),
            tz_datetime: format!("{}", now.format("%A %d %B @ %H:%M")),
            tz_found: false,
            source_url: "".to_owned(),
        };

        let start = now.with_hour(23).unwrap().with_minute(0).unwrap();
        let end = now.with_hour(7).unwrap().with_minute(0).unwrap();
        if now >= start || now <= end {
            r_response = RuleResponse { can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from("Use common sense"), ..r_response};
        }
        r_response
    }
}


#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};
    use chrono_tz::Asia::Beirut;

    use crate::rules::{rule::OtherTimezone, rule::Rulelike};

    #[test]
    fn test_other_timezone_rule() {
        let other_timezone = OtherTimezone{};

        // a timezone which is not explicitly implemented
        let tz = "Asia/Beirut".to_owned();

        let morning = Beirut.with_ymd_and_hms(2023, 11, 13, 08, 15, 00).unwrap();
        let morning_res = other_timezone.can_i_be_loud(morning.with_timezone(&Utc), tz.clone());
        assert_eq!(true, morning_res.can_i_be_loud);

        let afternoon = Beirut.with_ymd_and_hms(2023, 11, 13, 19, 15, 00).unwrap();
        let afternoon_res = other_timezone.can_i_be_loud(afternoon.with_timezone(&Utc), tz.clone());
        assert_eq!(true, afternoon_res.can_i_be_loud);

        let night = Beirut.with_ymd_and_hms(2023, 11, 13, 23, 15, 00).unwrap();
        let night_res = other_timezone.can_i_be_loud(night.with_timezone(&Utc), tz.clone());
        assert_eq!(false, night_res.can_i_be_loud);
    }
}
