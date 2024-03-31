use algorithms::fcfs::{FCFSScheduler, Process};
use std::io;

fn main() {
    let mut scheduler = fcfs::FCFSScheduler::new();
    let mut num_processes: i32 = 0;

    println!("Enter the number of processes: ");
    // ... (Input handling code - same as before) ...

    // Add processes to the scheduler
    for i in 0..num_processes {
        // ... (Read input data) ...

        let process = fcfs::Process { pid, arrival_time, burst_time };
        scheduler.add_process(process);
    }

    // Run the simulation
    scheduler.run();
}