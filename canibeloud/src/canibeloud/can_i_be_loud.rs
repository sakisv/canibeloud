pub enum CanIBeLoud {
    Yes,
    No,
}

impl CanIBeLoud {
    pub fn get_message(self) -> String {
        match self {
            CanIBeLoud::Yes => String::from("Yes (but within reason)"),
            CanIBeLoud::No => String::from("No"),
        }
    }
}
