use crate::process::Process;
use std::collections::BinaryHeap;

mod fcfs;
mod priority_non_preemptive;
mod priority_preemptive;
mod rr;
mod sjf;
mod srtf;

pub fn fcfs(processes: &mut Vec<Process>) -> Vec<& mut Process> {
    let mut current_time = 0;
    let mut finished_processes = Vec::new();

    for mut process in processes {
        current_time += process.burst_time;
        process.completion_time = Option::from(current_time);
        finished_processes.push(process);
    }

    finished_processes.clone() // Return a copy
}

pub fn sjf(processes: &mut Vec<Process>) -> Vec<Process> {
    let mut current_time = 0;
    let mut remaining_processes = processes.clone();
    let mut finished_processes = Vec::new();

    while !remaining_processes.is_empty() {
        remaining_processes.sort_by_key(|p| p.burst_time); // Sort by shortest burst time

        let mut next_process = remaining_processes.remove(0);
        current_time += next_process.burst_time;
        next_process.completion_time = Option::from(current_time);
        finished_processes.push(next_process);
    }

    finished_processes.clone()
}

pub fn priority(processes: &mut Vec<Process>) -> Vec<Process> {
    let mut priority_queue: BinaryHeap<Reverse<Process>> = BinaryHeap::new();
    let mut current_time = 0;
    let mut finished_processes = Vec::new();

    for p in processes {
        priority_queue.push(Reverse(p.clone()));
    }

    while let Some(Reverse(mut next_process)) = priority_queue.pop() {
        current_time += next_process.burst_time;
        next_process.completion_time = Option::from(current_time);
        finished_processes.push(next_process);
    }

    finished_processes.clone()
}

pub fn round_robin(processes: &mut Vec<Process>, time_quantum: i32) -> Vec<Process> {
    let mut current_time = 0;
    let mut remaining_processes = processes.clone();
    let mut finished_processes = Vec::new();

    while !remaining_processes.is_empty() {
        if let Some(mut next_process) = remaining_processes.pop() {
            if next_process.burst_time <= time_quantum as usize {
                current_time += next_process.burst_time;
                next_process.completion_time = Option::from(current_time);
                finished_processes.push(next_process);
            } else {
                next_process.burst_time -= time_quantum;
                current_time += time_quantum;
                remaining_processes.push(next_process);
            }
        }
    }

    finished_processes.clone()
}

// Wrapper for the priority queue
#[derive(PartialEq, Eq)]
struct Reverse<T>(T);

impl<T: Ord> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
