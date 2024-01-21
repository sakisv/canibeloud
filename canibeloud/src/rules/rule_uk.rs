// 11pm to 7am
// source: https://www.gov.uk/guidance/noise-nuisances-how-councils-deal-with-complaints

use chrono::{Local, Timelike as _};
use crate::canibeloud::can_i_be_loud::CanIBeLoud;

pub struct RuleUK {}

impl RuleUK {
    pub fn can_i_be_loud() -> CanIBeLoud {
        let now = Local::now();
        let start = now.with_hour(23).unwrap().with_minute(0).unwrap();
        let end = now.with_hour(7).unwrap().with_minute(0).unwrap();
        if now >= start && now <= end {
            return CanIBeLoud::No;
        }
        CanIBeLoud::Yes
    }
}
