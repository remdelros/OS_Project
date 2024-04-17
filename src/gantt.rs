use crate::process::Process;

pub fn generate_gantt_chart(processes: &Vec<&mut Process>) {
    // Sort processes by completion time for the visual representation
    let mut processes = processes.to_vec(); // Create a mutable copy
    processes.sort_by_key(|p| p.completion_time.unwrap_or(0));

    println!("----GANTT CHART----");
    generate_text_gantt_chart(processes);
}


fn generate_text_gantt_chart(processes: Vec<&mut Process>) {
    let max_completion_time = processes.last().unwrap().completion_time.unwrap_or(0);
    let num_processes = processes.len();

    // Top Timeline
    print!("  ");
    for i in 0..=max_completion_time {
        print!("{:>2}", i); // Align numbers
    }
    println!();

    // Process Blocks
    for (i, process) in processes.iter().enumerate() {
        print!("P{}|", process.pid); // Process label
        let start = process.arrival_time;
        let end = process.completion_time.unwrap_or(0);

        // Spaces before the process block
        for _ in 0..start {
            print!("  "); // Two spaces
        }

        // Process execution block
        for _ in 0..(end - start) {
            print!("██"); // Use a filled block character
        }
        println!();

        // Separator line (optional)
        if i < num_processes - 1 {
            print!("  ");
            for _ in 0..=max_completion_time {
                print!("--");
            }
            println!();
        }
    }
}
