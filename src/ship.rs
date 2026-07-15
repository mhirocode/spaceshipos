pub struct Ship {
    pub fuel: u32,
    pub power: u32,
    pub speed: u32,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            fuel: 100,
            power: 100,
            speed: 0,
        }
    }
}