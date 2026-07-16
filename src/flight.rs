use crate::task::Task;

pub struct FlightComputer;

impl FlightComputer {
    pub fn new() -> Self {
        Self
    }
}

impl Task for FlightComputer {
    fn update(&mut self) {
        println!("FlightComputer: running");
    }
}