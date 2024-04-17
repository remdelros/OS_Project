use std::io::{stdin, stdout, Write};
use crate::process::Process;

mod process;
mod algorithms;
mod gantt;

fn main() {

    loop {
        println!("Select an algorithm:");
        println!("1. FCFS (Non-Preemptive)");
        println!("2. SJF (Non-Preemptive)");
        println!("3. SRTF (Preemptive)");
        println!("4. Priority (Non-Preemptive)");
        println!("5. Priority (Preemptive)");
        println!("6. Round Robin");

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<usize>().unwrap();


        let mut num_processes: usize = 0;
        println!("Enter the number of processes: ");
        stdin().read_line(&mut num_processes).unwrap();
        let num_processes = num_processes.trim().parse::<usize>().unwrap();


        let mut processes = Vec::with_capacity(num_processes); // Pre-allocate space
        for i in 0..num_processes {
            println!("Enter arrival time and burst time for process {}:", i + 1);

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let mut values = input.trim().split_whitespace()
                .map(|s| s.parse::<usize>().unwrap());

            let arrival_time = values.next().unwrap();
            let burst_time = values.next().unwrap();

            processes.push(Process::new(i + 1, arrival_time, burst_time));
        }

        let result = match choice {
            1 => algorithms::fcfs(&mut processes),
            2 => algorithms::sjf(&mut processes),
            3 => algorithms::srtf(&mut processes),
            4 => algorithms::priority_nonpreemptive(&mut processes),
            5 => algorithms::priority_preemptive(&mut processes),
            6 => algorithms::round_robin(&mut processes),
            _ => panic!("Invalid choice"),
        };

        // 3. Display Gantt Chart
        gantt::generate_gantt_chart(&result);

        // 4. Display statistics (average waiting time, etc.)
        calculate_and_display_metrics(&result);

        // 5. Ask user to continue
        print!("Do you want to try again? (y/n): ");
        stdout().flush().unwrap(); // Ensure prompt is visible

        let mut again = String::new();
        stdin().read_line(&mut again).unwrap();

        if again.trim().to_lowercase() != "y" {
            break; // Exit the loop
        }
    }
}


