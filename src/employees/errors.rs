#[derive(Debug, PartialEq)]
pub struct VacationDaysShortageError {
    pub requested_days: u8,
    pub remaining_days: u8,
    pub message: String,
}

impl std::error::Error for VacationDaysShortageError {}

impl std::fmt::Display for VacationDaysShortageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Not enough vacation days are available. Requested: {}, Remaining: {}. Message: {}",
            self.requested_days, self.remaining_days, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacation_days_shortage_error() {
        let error = VacationDaysShortageError {
            requested_days: 10,
            remaining_days: 5,
            message: String::from("Not enough vacation days are available."),
        };

        assert_eq!(
            error.to_string(),
            "Not enough vacation days are available. Requested: 10, Remaining: 5. Message: Not enough vacation days are available."
        );
    }
}
