use crate::Process;
use core::{fmt, write};
use core::cmp::Ord;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};


#[derive(Clone, Debug, Eq, PartialEq)]

pub struct GanttEntry {
    process_id: String,
    start_time: usize,
    end_time: usize,
}

impl fmt::Display for crate::priority::GanttEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}-{}]", self.process_id, self.start_time, self.end_time)
    }
}

pub mod priority {
    use super::*;

    pub fn scheduling(processes: &[Process]) -> Vec<GanttEntry> {
        let mut processes = processes.to_vec();
        for process in &mut processes {
            process.remaining_time = process.burst_time; // Initialize remaining time
        }
        processes.sort_by_key(|p| p.arrival_time);

        let mut gantt_chart = Vec::new();
        let mut ready_queue: BinaryHeap<Process> = BinaryHeap::new();
        let mut current_time = 0;
        let mut current_process: Option<Process> = None;

        let mut i = 0;
        while i < processes.len() || current_process.is_some() || !ready_queue.is_empty() {
            while i < processes.len() && processes[i].arrival_time <= current_time {
                ready_queue.push(processes[i].clone());
                i += 1;
            }

            if let Some(mut top_process) = ready_queue.pop() {
                if current_process.is_none() || top_process.priority < current_process.as_ref().unwrap().priority {
                    if let Some(mut running_process) = current_process.take() {
                        gantt_chart.push(GanttEntry {
                            process_id: running_process.id.clone(),
                            start_time: current_time - running_process.remaining_time,
                            end_time: current_time,
                        });
                        running_process.remaining_time = 0;
                        ready_queue.push(running_process);
                    }
                    current_process = Some(top_process);
                }
            }

            if let Some(ref mut process) = current_process {
                process.remaining_time -= 1;
                current_time += 1;

                if process.remaining_time == 0 {
                    gantt_chart.push(GanttEntry {
                        process_id: process.id.clone(),
                        start_time: current_time - 1,
                        end_time: current_time,
                    });
                    current_process = None;
                }
            } else {
                // No process is ready, increment time until the next arrival
                if i < processes.len() {
                    current_time = processes[i].arrival_time;
                } else {
                    current_time += 1;
                }
            }
        }

        gantt_chart
    }
}