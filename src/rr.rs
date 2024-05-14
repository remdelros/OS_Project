use core::{fmt, write};
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};
use crate::process::Process;
use crate::GanttEntry;

pub mod rr {
    use std::collections::VecDeque;
    use super::*;

    pub fn scheduling(processes: &[Process], time_quantum: usize) -> Vec<GanttEntry> {
        let mut processes = processes.to_vec();
        for process in &mut processes {
            process.remaining_time = process.burst_time;
        }
        processes.sort_by_key(|p| p.arrival_time);

        let mut gantt_chart = Vec::new();
        let mut queue: VecDeque<Process> = VecDeque::new();
        let mut current_time = 0;

        for process in processes {
            queue.push_back(process);
        }

        while !queue.is_empty() {
            let mut process = queue.pop_front().unwrap();

            let execution_time = if process.remaining_time > time_quantum {
                time_quantum
            } else {
                process.remaining_time
            };

            gantt_chart.push(GanttEntry {
                process_id: process.id.clone(),
                start_time: current_time,
                end_time: current_time + execution_time,
            });
            current_time += execution_time;
            process.remaining_time -= execution_time;

            // Check for new arrivals during the quantum
            while !queue.is_empty() && queue[0].arrival_time <= current_time {
                let temp_process = queue.pop_front().unwrap(); // Fix: Temporary variable
                queue.push_back(temp_process);
            }

            if process.remaining_time > 0 {
                queue.push_back(process);
            }
        }

        gantt_chart
    }
}