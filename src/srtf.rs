use core::{fmt, write};
use std::collections::BinaryHeap;
use std::cmp::{Ordering};
use core::clone::Clone;
use core::cmp::{Ord, PartialOrd, Reverse};
use core::option::Option;
use newtype_derive;
use crate::process::Process;
use crate::GanttEntry;

// Newtype wrapper around Reverse<Process>
struct ReversedProcess(Reverse<Process>);


pub mod srtf {
    use super::*;

    pub fn scheduling(processes: &[Process]) -> Vec<GanttEntry> {
        let mut processes = processes.to_vec();
        for process in &mut processes {
            process.remaining_time = process.burst_time;
        }
        processes.sort_by_key(|p| p.arrival_time);

        let mut gantt_chart = Vec::new();
        let mut ready_queue: BinaryHeap<Reverse<Process>> = BinaryHeap::new();
        let mut current_time = 0;
        let mut current_process: Option<Process> = None;

        let mut i = 0;
        while i < processes.len() || current_process.is_some() || !ready_queue.is_empty() {
            while i < processes.len() && processes[i].arrival_time <= current_time {
                ready_queue.push(Reverse(processes[i].clone()));
                i += 1;
            }

            if let Some(Reverse(top_process)) = ready_queue.peek() {
                if current_process.is_none() || top_process.remaining_time < current_process.as_ref().unwrap().remaining_time {
                    if let Some(mut running_process) = current_process.take() {
                        gantt_chart.push(GanttEntry {
                            process_id: running_process.id.clone(),
                            start_time: current_time - running_process.remaining_time,
                            end_time: current_time,
                        });
                        ready_queue.push(Reverse(running_process));
                    }
                    current_process = Some(ready_queue.pop().unwrap().0);
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