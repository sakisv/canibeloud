// 1/10 - 31/3: 15:30-17:30 && 22:00-07:30
// 1/4 - 30/9: 15:00-17:30 && 23:00-07:00
// source:
// https://www.astynomia.gr/odigos-tou-politi/chrisimes-symvoules/diafores/poies-einai-oi-ores-koinis-isychias/

use chrono::{Local, Timelike as _, Datelike as _};
use crate::canibeloud::can_i_be_loud::CanIBeLoud;

#[derive()]
pub struct RuleGR {}

impl RuleGR {
    pub fn can_i_be_loud() -> CanIBeLoud {
        let now = Local::now();
        match now.month() {
            4..=9 => {
                let start_noon = now.with_hour(15).unwrap().with_minute(0).unwrap();
                let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

                let start_night = now.with_hour(23).unwrap().with_minute(0).unwrap();
                let end_night = now.with_hour(7).unwrap().with_minute(0).unwrap();
                if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                    return CanIBeLoud::No;
                }
            }
            _ => {
                let start_noon = now.with_hour(15).unwrap().with_minute(30).unwrap();
                let stop_noon = now.with_hour(17).unwrap().with_minute(30).unwrap();

                let start_night = now.with_hour(22).unwrap().with_minute(0).unwrap();
                let end_night = now.with_hour(7).unwrap().with_minute(30).unwrap();
                if (now >= start_noon && now <= stop_noon) || (now >= start_night || now <= end_night) {
                    return CanIBeLoud::No;
                }
            }
        }
        CanIBeLoud::Yes
    }
}
