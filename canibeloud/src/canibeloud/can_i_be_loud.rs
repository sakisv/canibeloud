use serde::Serialize;
#[derive(Serialize)]
pub struct CanIBeLoudResponse {
    pub can_i_be_loud: bool,
    pub response_text: String,
    pub secondary_text: String,
    pub requested_timezone: String,
    pub timezone_found: bool,
    pub tz_datetime: String,
    pub source: String,
}
