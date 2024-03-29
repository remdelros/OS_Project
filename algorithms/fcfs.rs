struct Process {
    pid: i32,
    arrival_time: i32,
    burst_time: i32,
}

fn fcfs(processes: &mut Vec<Process>) {
    let mut current_time = 0;
    let mut completed = 0;
    let mut ready_queue: Vec<&Process> = Vec::new();

    while completed != processes.len() {
        // Add newly arriving processes to the ready queue
        for process in processes.iter() {
            if process.arrival_time <= current_time && !ready_queue.contains(&process) {
                ready_queue.push(process);
            }
        }

        // If a process is running, let it continue
        if let Some(running_process) = ready_queue.first() {
            current_time += running_process.burst_time;
            let index = processes
                .iter()
                .position(|p| p.pid == running_process.pid)
                .unwrap();

            // Calculate metrics for the completed process
            let turnaround_time = current_time - processes[index].arrival_time;
            let waiting_time = turnaround_time - processes[index].burst_time;
            println!(
                "Process {} completed: Turnaround Time = {}, Waiting Time = {}",
                processes[index].pid, turnaround_time, waiting_time
            );

            completed += 1;
            ready_queue.remove(0); // Remove the finished process
        } else {
            current_time += 1; // If nothing is running, increment time
        }
    }
}

fn main() {
    let mut processes = vec![
        Process { pid: 1, arrival_time: 0, burst_time: 5 },
        Process { pid: 2, arrival_time: 2, burst_time: 2 },
        Process { pid: 3, arrival_time: 1, burst_time: 8 },
    ];

    fcfs(&mut processes);
}