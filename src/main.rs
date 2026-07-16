mod scheduler;
mod task;
mod flight;

use scheduler::Scheduler;
use flight::FlightComputer;

fn main() {
    println!("ShipOS Boot Sequence");

    let mut scheduler = Scheduler::new();

    scheduler.add_task(FlightComputer::new());

    for tick in 1..=5 {
        println!("\nTick {}", tick);
        scheduler.update();
    }
}