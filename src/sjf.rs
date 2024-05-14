use core::{fmt, write};
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};
use crate::process::Process;
use crate::GanttEntry;


impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare burst_time (shortest job first)
        self.burst_time.cmp(&other.burst_time)
            // If burst times are equal, compare arrival times
            .then_with(|| self.arrival_time.cmp(&other.arrival_time))
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub mod sjf {
    use super::*;

    pub fn scheduling(processes: &[Process]) -> Vec<GanttEntry> {
        let mut processes = processes.to_vec();
        for process in &mut processes {
            process.remaining_time = process.burst_time; // Initialize remaining time
        }
        processes.sort_by_key(|p| p.arrival_time);

        let mut gantt_chart = Vec::new();
        let mut ready_queue: BinaryHeap<Process> = BinaryHeap::new(); // Min-heap based on burst time
        let mut current_time = 0;
        let mut i = 0;

        while i < processes.len() || !ready_queue.is_empty() {
            while i < processes.len() && processes[i].arrival_time <= current_time {
                ready_queue.push(processes[i].clone());
                i += 1;
            }

            if let Some(mut process) = ready_queue.pop() {
                gantt_chart.push(GanttEntry {
                    process_id: process.id.clone(),
                    start_time: current_time,
                    end_time: current_time + process.burst_time,
                });
                current_time += process.burst_time;
            }
            else{
                current_time = processes[i].arrival_time;
            }

        }

        gantt_chart
    }
}