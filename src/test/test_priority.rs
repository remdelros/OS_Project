use crate::algorithms::priority; // Assuming you have a priority scheduling function
use crate::process::Process;

#[test]
fn test_priority_basic() {
    let mut processes = vec![
        Process { priority: Some(2), ..Process::new() }, // Arrival 0, Burst 5, Priority 2
        Process { priority: Some(1), ..Process::new() }, // Arrival 1, Burst 8, Priority 1
        Process { priority: Some(3), ..Process::new() }, // Arrival 3, Burst 3, Priority 3
    ];

    let results = priority(&mut processes);

    // Assert highest priority (lowest value) finishes first
    assert_eq!(results[0].pid, processes[2].pid);
    assert_eq!(results[1].pid, processes[0].pid);
    assert_eq!(results[2].pid, processes[1].pid);

    // Add assertions on turnaround and waiting time, considering priorities
}

#[test]
fn test_priority_tiebreaker() {
    // Test case where multiple processes have the same priority--
    // ensure FCFS behavior is applied within those priority levels
}
