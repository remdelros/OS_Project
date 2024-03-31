use std::process::Command;  // For running Linux commands
mod algorithms;
mod process;
mod metrics;
mod algorithms;
mod process;
mod metrics;

fn main() {
    let mut processes = process::generate_processes(20); // Generate some sample processes

    // Example: Running FCFS scheduling
    let results = algorithms::fcfs(&mut processes);

    // Collect metrics
    let turnaround_time = metrics::calculate_turnaround_time(&results);
    let waiting_time = metrics::calculate_waiting_time(&results);

    println!("FCFS Results:");
    println!("Average Turnaround Time: {}", turnaround_time);
    println!("Average Waiting Time: {}", waiting_time);
}
