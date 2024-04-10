use rand::Rng;

#[derive(Debug, Clone)]
struct Process {
    pid: usize,
    arrival_time: usize,
    burst_time: usize,
    priority: Option<usize>,
    remaining_time: usize,
    completion_time: Option<usize>,
    waiting_time: Option<usize>,
    turnaround_time: Option<usize>,
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
