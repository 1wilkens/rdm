use crate::seat::Seat;

#[derive(Debug)]
pub struct SeatManager {
    seats: Vec<Seat>
}

impl SeatManager {
    pub fn new() -> Self {
        SeatManager {
            seats: Vec::new()
        }
    }

    pub fn add_seat(&mut self, name: &str) {
        self.seats.push(Seat::new(name));
    }
}