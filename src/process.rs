use rand::Rng;

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: i32,
    pub arrival_time: i32,
    pub burst_time: i32,
    pub priority: Option<i32>, //  If applicable
}

impl Process {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Process {
            pid: rng.gen(),
            arrival_time: rng.gen_range(0..10),
            burst_time: rng.gen_range(5..20),
            priority: None, // Adjust for priority scheduling
        }
    }
}
