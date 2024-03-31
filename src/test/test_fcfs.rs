use crate::algorithms::fcfs;
use crate::process::Process;

#[test]
fn test_fcfs_basic() {
    let mut processes = vec![
        Process::new(), // Arrival 0, Burst 5
        Process::new(), // Arrival 1, Burst 8
        Process::new(), // Arrival 3, Burst 3
    ];

    let results = fcfs(&mut processes);

    // Assert finishing order (remember, FCFS is non-preemptive)
    assert_eq!(results[0].pid, processes[0].pid);
    assert_eq!(results[1].pid, processes[1].pid);
    assert_eq!(results[2].pid, processes[2].pid);

    // You would further assert turnaround times and waiting
    // times based on calculations within fcfs
}
