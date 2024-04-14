use std::io::{stdin, stdout, Write};

mod process;
mod algorithms;
mod gantt;
mod gantt;
mod priority;

fn main() {
    // ... (Existing code from previous example)

    loop {
        println!("Select an algorithm:");
        println!("1. FCFS (Non-Preemptive)");
        println!("2. SJF (Non-Preemptive)");
        println!("3. SRTF (Preemptive)");
        println!("4. Priority (Non-Preemptive)");
        println!("5. Priority (Preemptive)");
        println!("6. Round Robin");

        // ... (Input collection as before)

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
