// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

#[derive(Clone, Debug)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}


#[cfg(test)]
mod tests {
    use crate::{summary, Ticket};

    #[test]
    fn test1() {
        let t = Ticket {
            title: "1".to_string(),
            description: "2".to_string(),
            status: "3".to_string(),
        };
        println!("{:?}", t);
        let (t2, s1) = summary(t);
        println!("{:?}", t2);
    }
}