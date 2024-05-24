// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use std::fmt::{Display, Formatter};
use crate::Status::{Done, InProgress, ToDo};
use crate::TicketNewError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Status {
    fn values() -> Vec<Status> {
        return vec![ToDo, InProgress, Done];
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ToDo => { "todo" }
            InProgress => { "inProgress" }
            Done => { "done" }
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<String> for Status {
    type Error = TicketNewError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for s in Self::values() {
            if (s.to_string().eq_ignore_ascii_case(value.as_str())) {
                return Ok(s);
            }
        }
        return Err(TicketNewError::StatusInvalid);
    }
}

impl TryFrom<&str> for Status {
    type Error = TicketNewError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for s in Self::values() {
            if (s.to_string().eq_ignore_ascii_case(value)) {
                return Ok(s);
            }
        }
        return Err(TicketNewError::StatusInvalid);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
