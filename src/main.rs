extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;
use std::io;
use std::fmt;
use std::collections::{BinaryHeap, VecDeque};
use std::cmp::{Ordering, Reverse};

// Import scheduling modules
mod fcfs;
mod sjf;
mod srtf;
mod rr;
mod priority;
mod process;

use process::{Process, MyProcess};

// Common GanttEntry struct (used by all modules)
#[derive(Debug)]
struct GanttEntry {
    process_id: String,
    start_time: usize,
    end_time: usize,
}

/*impl fmt::Display for GanttEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}-{}]", self.process_id, self.start_time, self.end_time)
    }
}*/

fn main() {
    println!("Choose a CPU scheduling algorithm:");
    println!("1. FCFS (First Come First Served)");
    println!("2. SJF (Shortest Job First)");
    println!("3. SRTF (Shortest Remaining Time First)");
    println!("4. Round Robin");
    println!("5. Priority");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input.");

    let choice: u32 = choice.trim().parse().expect("Invalid input. Please enter a number.");

    let mut processes = Vec::new();
    let mut time_quantum = 0;

    println!("Enter the number of processes:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input.");
    let n: usize = n.trim().parse().expect("Invalid input. Please enter a number.");

    for i in 0..n {
        println!("Enter arrival time, burst time, and priority (if applicable) for process P{}:", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.trim().parse().expect("Invalid input. Please enter numbers."))
            .collect();

        let (arrival_time, burst_time) = (input[0], input[1]);

        let process = if choice == 5 {
            let priority = input[2];
            Process { id: format!("P{}", i + 1), arrival_time, burst_time, priority, remaining_time: burst_time }
        } else {
            Process { id: format!("P{}", i + 1), arrival_time, burst_time, priority: 0, remaining_time: burst_time }
        };

        processes.push(process);

        if choice == 4 { // Round Robin
            println!("Enter time quantum for Round Robin:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input.");
            time_quantum = input.trim().parse().expect("Invalid input. Please enter a number.");
        }
    }



    let gantt_chart = match choice {
        1 => fcfs::fcfs::scheduling(&processes),
        2 => sjf::sjf::scheduling(&processes),
        3 => srtf::srtf::scheduling(&processes),
        4 => rr::rr::scheduling(&processes, time_quantum),
        5 => {
            let mut wrapped_processes: Vec<MyProcess> = processes.iter().map(|p| MyProcess(Rc::new(RefCell::new(p.clone())))).collect();
            let gantt_chart = priority::priority::scheduling(&wrapped_processes);
            gantt_chart.into_iter().map(|entry| GanttEntry {
                process_id: entry.process_id,
                start_time: entry.start_time,
                end_time: entry.end_time
            }).collect()
        },
        _ => panic!("Invalid choice."),
    };

    println!("\nGantt Chart:");
    for entry in gantt_chart {
        println!("{}", entry);
    }
}
