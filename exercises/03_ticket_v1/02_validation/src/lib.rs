use std::{panic, str::FromStr};
enum Status {
    Todo,
    InProgress,
    Done,
}
impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "To-Do" => Ok(Status::Todo),
            "In Progress" => Ok(Status::InProgress),
            "Done" => Ok(Status::Done),
            _ => Err(()),
        }
    }
}
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        let status_enum = Status::from_str(&status)
            .expect("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");

        if description.is_empty() {
            panic!("Description cannot be empty");
        } else if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        if title.is_empty() {
            panic!("Title cannot be empty");
        } else if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }

        Self {
            title,
            description,
            status: status_enum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
