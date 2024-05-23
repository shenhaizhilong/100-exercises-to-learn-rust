// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::fmt::{Display, Formatter};
use crate::Status::{Done, InProgress, ToDo};

// extern crate enum_iterator;

#[derive(Debug, PartialEq, Clone)]
enum Status {
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
        let v = match self {
            Status::ToDo => { "todo" }
            Status::InProgress => { "inProgress" }
            Status::Done => { "done" }
        };
        write!(f, "{}", v)
    }
}

impl TryFrom<String> for Status {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for s in Self::values() {
            if s.to_string().eq_ignore_ascii_case(&value) {
                return Ok(s);
            }
        }
        panic!("not a status");
    }
}

impl TryFrom<&str> for Status {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for s in Self::values() {
            if s.to_string().eq_ignore_ascii_case(value) {
                return Ok(s);
            }
        }
        panic!("not a status");
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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
