use core::{fmt, write};
use crate::process::Process;
use crate::GanttEntry;


impl fmt::Display for crate::fcfs::GanttEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}-{}]", self.process_id, self.start_time, self.end_time)
    }
}

pub mod fcfs {
    use alloc::vec::Vec;
    use super::*;

    pub fn scheduling(processes: &[Process]) -> Vec<GanttEntry> {
        let mut processes = processes.to_vec();
        for process in &mut processes {
            process.remaining_time = process.burst_time;
        }
        processes.sort_by_key(|p| p.arrival_time);

        let mut gantt_chart = Vec::new();
        let mut current_time = 0;
        let mut total_waiting_time = 0;

        for process in processes {
            if process.arrival_time > current_time {
                current_time = process.arrival_time;
            }

            total_waiting_time += current_time - process.arrival_time;
            gantt_chart.push(GanttEntry {
                process_id: process.id.clone(),
                start_time: current_time,
                end_time: current_time + process.burst_time,
            });

            current_time += process.burst_time;
        }

        gantt_chart
    }
}