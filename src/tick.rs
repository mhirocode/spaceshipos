pub struct Tick {
    count: u64,
}

impl Tick {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn next(&mut self) {
        self.count += 1;
        println!("Tick {}", self.count);
    }
}