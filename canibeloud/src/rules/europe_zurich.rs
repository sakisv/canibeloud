// 22:00 - 06:00
// 12:00 - 13:00
// Sundays
// Public holidays
// source:
// https://www.ch.ch/en/housing/rent/noise--damage-and-rent#designated-quiet-times-in-switzerland

use chrono::{DateTime, Datelike, Timelike as _, Utc, Weekday};
use super::rule::{Rulelike, RuleResponse};
use chrono_tz::Tz;

pub struct EuropeZurich {}

impl Rulelike for EuropeZurich {
    fn can_i_be_loud(&self, utc_now: DateTime<Utc>, _: String) -> RuleResponse {
        let athens_tz: Tz = "Europe/Zurich".parse().unwrap();
        let now = utc_now.with_timezone(&athens_tz);

        let r_response = RuleResponse {
            can_i_be_loud: true,
            response_text: String::from("Yes"),
            secondary_text: String::from("(But within reason)"),
            tz_datetime: format!("{}", now.format("%A %d %B @ %H:%M")),
            tz_found: true,
            source_url: "https://www.ch.ch/en/housing/rent/noise--damage-and-rent#designated-quiet-times-in-switzerland".to_owned(),
        };

        // Sundays are quiet
        if now.weekday() == Weekday::Sun {
            return RuleResponse{can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from("It's Sunday"), ..r_response};
        }

        // Public holidays are also quiet but the differ between cantons. The common ones are:
        // 1st Jan, 1st Aug, 25th Dec
        // Sources:
        // https://www.ch.ch/en/work/working-hours/vacation--public-holidays-and-absences-from-work#public-holidays
        // https://en.wikipedia.org/wiki/Public_holidays_in_Switzerland
        if (now.month() == 1 && now.day() == 1) || (now.month() == 8 && now.day() == 1) || (now.month() == 12 && now.day() == 25) {
            return RuleResponse{can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from("It's a holiday"), ..r_response};
        }

        // Nights are quiet
        let start_night = now.with_hour(22).unwrap().with_minute(0).unwrap();
        let end_night = now.with_hour(6).unwrap().with_minute(0).unwrap();
        if now >= start_night || now <= end_night {
            return RuleResponse{can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from("It's night ffs"), ..r_response};
        }

        // Noons are quiet
        let start_noon = now.with_hour(12).unwrap().with_minute(0).unwrap();
        let stop_noon = now.with_hour(13).unwrap().with_minute(0).unwrap();
        if now >= start_noon && now <= stop_noon {
            return RuleResponse{can_i_be_loud: false, response_text: String::from("No"), secondary_text: String::from("It's noon"), ..r_response};
        }

        r_response
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};
    use chrono_tz::Europe::Zurich;

    use crate::rules::{europe_zurich::EuropeZurich, rule::Rulelike};

    #[test]
    fn test_europe_zurich_rule() {
        let europe_zurich = EuropeZurich{};
        let tz = "Europe/Zurich".to_owned();

        // Sunday
        let sunday_morning = Zurich.with_ymd_and_hms(2024, 3, 24, 10, 15, 00).unwrap();
        let sunday_morning_res = europe_zurich.can_i_be_loud(sunday_morning.with_timezone(&Utc), tz.clone());
        assert_eq!(false, sunday_morning_res.can_i_be_loud);
        assert_eq!("It's Sunday", sunday_morning_res.secondary_text);

        // Holiday
        let xmas_morning = Zurich.with_ymd_and_hms(2024, 12, 25, 10, 15, 00).unwrap();
        let xmas_morning_res = europe_zurich.can_i_be_loud(xmas_morning.with_timezone(&Utc), tz.clone());
        assert_eq!(false, xmas_morning_res.can_i_be_loud);
        assert_eq!("It's a holiday", xmas_morning_res.secondary_text);

        // night
        let night = Zurich.with_ymd_and_hms(2024, 3, 22, 23, 15, 00).unwrap();
        let night_res = europe_zurich.can_i_be_loud(night.with_timezone(&Utc), tz.clone());
        assert_eq!(false, night_res.can_i_be_loud);
        assert_eq!("It's night ffs", night_res.secondary_text);

        // noon
        let noon = Zurich.with_ymd_and_hms(2024, 3, 22, 12, 15, 00).unwrap();
        let noon_res = europe_zurich.can_i_be_loud(noon.with_timezone(&Utc), tz.clone());
        assert_eq!(false, noon_res.can_i_be_loud);
        assert_eq!("It's noon", noon_res.secondary_text);

        // non-quiet time
        let noon = Zurich.with_ymd_and_hms(2024, 3, 22, 15, 15, 00).unwrap();
        let noon_res = europe_zurich.can_i_be_loud(noon.with_timezone(&Utc), tz.clone());
        assert_eq!(true, noon_res.can_i_be_loud);
        assert_eq!("(But within reason)", noon_res.secondary_text);
    }
}
