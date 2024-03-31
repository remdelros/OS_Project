use crate::process::Process;

pub fn calculate_turnaround_time(processes: &Vec<Process>) -> f32 {
    let total_turnaround_time: i32 = processes
        .iter()
        .map(|p| p.completion_time - p.arrival_time)
        .sum();

    (total_turnaround_time as f32) / (processes.len() as f32)
}

pub fn calculate_waiting_time(processes: &Vec<Process>) -> f32 {
    let total_waiting_time: i32 = processes
        .iter()
        .map(|p| p.completion_time - p.arrival_time - p.burst_time)
        .sum();

    (total_waiting_time as f32) / (processes.len() as f32)
}

pub fn calculate_cpu_utilization(processes: &Vec<Process>, total_time: i32) -> f32 {
    let total_burst_time: i32 = processes.iter().map(|p| p.burst_time).sum();

    (total_burst_time as f32 / total_time as f32) * 100.0 // Expressed as a percentage
}
