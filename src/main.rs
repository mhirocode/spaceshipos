mod tick;

use tick::Tick;
use std::{thread, time::Duration};

fn main() {
    println!("======================================");
    println!("         ShipOS Boot Sequence");
    println!("======================================");
    println!("Initializing spacecraft systems...");
    println!("System ready.\n");

    let mut tick = Tick::new();

    loop {
        tick.next();
        thread::sleep(Duration::from_millis(1000));
    }
}