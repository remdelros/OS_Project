use crate::process::{Process, MyProcess};
use crate::GanttEntry;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

pub mod priority {
    use alloc::rc::Rc;
    use core::cell::RefCell;
    use core::option::Option::Some;
    use super::*;

    pub fn scheduling(processes: &[MyProcess]) -> Vec<GanttEntry> { // change input type to MyProcess
        let mut processes = processes.to_vec();
        processes.sort_by_key(|p| p.0.borrow().arrival_time); // Sort based on wrapped Process's arrival time

        let mut gantt_chart = Vec::new();
        let mut ready_queue: BinaryHeap<MyProcess> = BinaryHeap::new(); // Min-heap based on priority
        let mut current_time = 0;
        let mut current_process: Option<Process> = None;

        let mut i = 0;
        while i < processes.len() || current_process.is_some() || !ready_queue.is_empty() {
            while i < processes.len() && processes[i].0.borrow().arrival_time <= current_time {
                ready_queue.push(processes[i].clone());
                i += 1;
            }

            // Check if there's a top process, get it, and immediately drop the reference
            if let Some(top_process) = ready_queue.peek() {
                // if the current_process is None, or if there is a current process but the top process has higher priority, swap the current process with the top process.
                if current_process.is_none() || top_process.0.borrow().priority < current_process.as_ref().unwrap().priority {

                    // If there's a running process, stop it and add it back to the queue
                    if let Some(mut running_process) = current_process.take() {
                        gantt_chart.push(GanttEntry {
                            process_id: running_process.id.clone(),
                            start_time: current_time - running_process.remaining_time,
                            end_time: current_time,
                        });

                        running_process.remaining_time = 0;
                        ready_queue.push(MyProcess(Rc::new(RefCell::new(running_process))));
                    }

                    current_process = Some(Rc::try_unwrap(ready_queue.pop().unwrap().0).unwrap().into_inner()); // Remove the top process from the ready queue and start running it
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
                    current_time = processes[i].0.borrow().arrival_time;
                } else {
                    current_time += 1;
                }
            }
        }

        gantt_chart
    }
}

