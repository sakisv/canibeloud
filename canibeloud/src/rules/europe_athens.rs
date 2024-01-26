// 1/10 - 31/3: 15:30-17:30 && 22:00-07:30
// 1/4 - 30/9: 15:00-17:30 && 23:00-07:00
// source:
// https://www.astynomia.gr/odigos-tou-politi/chrisimes-symvoules/diafores/poies-einai-oi-ores-koinis-isychias/

use chrono::{DateTime, Utc, Timelike as _, Datelike as _};
use super::rule::{Rulelike, RuleResponse};
use chrono_tz::Tz;

pub struct EuropeAthens {}

impl Rulelike for EuropeAthens {
    fn can_i_be_loud(&self, utc_now: DateTime<Utc>, _: String) -> RuleResponse {
        let athens_tz: Tz = "Europe/Athens".parse().unwrap();
        let now = utc_now.with_timezone(&athens_tz);

        let mut r_response = RuleResponse {
            can_i_be_loud: true,
            response_text: String::from("Ναι"),
            secondary_text: String::from("(Αλλά με μέτρο)"),
            tz_datetime: format!("Είναι {}", now.format("%H:%M")),
            tz_found: true,
            source_url: "https://www.astynomia.gr/odigos-tou-politi/chrisimes-symvoules/diafores/poies-einai-oi-ores-koinis-isychias/".to_owned(),
        };

        match now.month() {
            4..=9 => {
                let start_noon = now.with_hour(15).unwrap().with_minute(0).unwrap();
                let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

                let start_night = now.with_hour(23).unwrap().with_minute(0).unwrap();
                let end_night = now.with_hour(7).unwrap().with_minute(0).unwrap();
                if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                    r_response = RuleResponse{can_i_be_loud: false, response_text: String::from("Όχι"), secondary_text: String::from(""), ..r_response};
                }
            }
            _ => {
                let start_noon = now.with_hour(15).unwrap().with_minute(30).unwrap();
                let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

                let start_night = now.with_hour(22).unwrap().with_minute(0).unwrap();
                let end_night = now.with_hour(7).unwrap().with_minute(30).unwrap();
                if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                    r_response = RuleResponse{can_i_be_loud: false, response_text: String::from("Όχι"), secondary_text: String::from(""), ..r_response};
                }
            }
        }
        r_response
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};
    use chrono_tz::Europe::Athens;

    use crate::rules::{europe_athens::EuropeAthens, rule::Rulelike};

    #[test]
    fn test_europe_athens_rule() {
        let europe_athens = EuropeAthens{};
        let tz = "Europe/Athens".to_owned();

        // winter

        let winter_morning = Athens.with_ymd_and_hms(2023, 11, 13, 10, 15, 00).unwrap();
        let winter_morning_res = europe_athens.can_i_be_loud(winter_morning.with_timezone(&Utc), tz.clone());
        assert_eq!(true, winter_morning_res.can_i_be_loud);

        let winter_noon = Athens.with_ymd_and_hms(2023, 11, 13, 16, 15, 00).unwrap();
        let winter_noon_res = europe_athens.can_i_be_loud(winter_noon.with_timezone(&Utc), tz.clone());
        assert_eq!(false, winter_noon_res.can_i_be_loud);

        let winter_evening = Athens.with_ymd_and_hms(2023, 11, 13, 19, 15, 00).unwrap();
        let winter_evening_res = europe_athens.can_i_be_loud(winter_evening.with_timezone(&Utc), tz.clone());
        assert_eq!(true, winter_evening_res.can_i_be_loud);

        let winter_night = Athens.with_ymd_and_hms(2023, 11, 13, 23, 15, 00).unwrap();
        let winter_night_res = europe_athens.can_i_be_loud(winter_night.with_timezone(&Utc), tz.clone());
        assert_eq!(false, winter_night_res.can_i_be_loud);


        // summer

        let summer_morning = Athens.with_ymd_and_hms(2023, 07, 13, 10, 15, 00).unwrap();
        let summer_morning_res = europe_athens.can_i_be_loud(summer_morning.with_timezone(&Utc), tz.clone());
        assert_eq!(true, summer_morning_res.can_i_be_loud);

        let summer_noon = Athens.with_ymd_and_hms(2023, 07, 13, 16, 15, 00).unwrap();
        let summer_noon_res = europe_athens.can_i_be_loud(summer_noon.with_timezone(&Utc), tz.clone());
        assert_eq!(false, summer_noon_res.can_i_be_loud);

        let summer_evening = Athens.with_ymd_and_hms(2023, 07, 13, 19, 15, 00).unwrap();
        let summer_evening = europe_athens.can_i_be_loud(summer_evening.with_timezone(&Utc), tz.clone());
        assert_eq!(true, summer_evening.can_i_be_loud);

        let summer_night = Athens.with_ymd_and_hms(2023, 07, 13, 23, 15, 00).unwrap();
        let summer_night_res = europe_athens.can_i_be_loud(summer_night.with_timezone(&Utc), tz.clone());
        assert_eq!(false, summer_night_res.can_i_be_loud);

        // differences (morning @ 07:15)
        let summer_early_morning = Athens.with_ymd_and_hms(2023, 07, 13, 07, 15, 00).unwrap();
        let winter_early_morning = Athens.with_ymd_and_hms(2023, 11, 13, 07, 15, 00).unwrap();
        let summer_early_morning_res = europe_athens.can_i_be_loud(summer_early_morning.with_timezone(&Utc), tz.clone());
        let winter_early_morning_res = europe_athens.can_i_be_loud(winter_early_morning.with_timezone(&Utc), tz.clone());
        assert_eq!(true, summer_early_morning_res.can_i_be_loud);
        assert_eq!(false, winter_early_morning_res.can_i_be_loud);

        // differences (noon @ 15:15)
        let summer_early_noon = Athens.with_ymd_and_hms(2023, 07, 13, 15, 15, 00).unwrap();
        let winter_early_noon = Athens.with_ymd_and_hms(2023, 11, 13, 15, 15, 00).unwrap();
        let summer_early_noon_res = europe_athens.can_i_be_loud(summer_early_noon.with_timezone(&Utc), tz.clone());
        let winter_early_noon_res = europe_athens.can_i_be_loud(winter_early_noon.with_timezone(&Utc), tz.clone());
        assert_eq!(false, summer_early_noon_res.can_i_be_loud);
        assert_eq!(true, winter_early_noon_res.can_i_be_loud);

        // differences (night @ 22:15)
        let summer_early_night = Athens.with_ymd_and_hms(2023, 07, 13, 22, 15, 00).unwrap();
        let winter_early_night = Athens.with_ymd_and_hms(2023, 11, 13, 22, 15, 00).unwrap();
        let summer_early_night_res = europe_athens.can_i_be_loud(summer_early_night.with_timezone(&Utc), tz.clone());
        let winter_early_night_res = europe_athens.can_i_be_loud(winter_early_night.with_timezone(&Utc), tz.clone());
        assert_eq!(true, summer_early_night_res.can_i_be_loud);
        assert_eq!(false, winter_early_night_res.can_i_be_loud);
    }
}
