#[derive(Debug)]
pub struct Seat {
    name: String
}

impl Seat {
    pub fn new(name: &str) -> Seat {
        Seat {
            name: name.to_owned()
        }
    }
}