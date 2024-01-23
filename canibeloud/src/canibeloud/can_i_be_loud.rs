use serde::Serialize;
#[derive(Serialize)]
pub struct CanIBeLoudResponse {
    can_i_be_loud: bool,
    response_text: String,
    requested_timezone: String,
    timezone_found: bool,
    calculated_datetime: String,
}
