use crate::algorithms::sjf; // Assuming you implement the SJF function
use crate::process::Process;

#[test]
fn test_sjf_preemptive() {
    let mut processes = vec![
        Process::new(), // Arrival 0, Burst 5
        Process::new(), // Arrival 1, Burst 8
        Process::new(), // Arrival 3, Burst 3
    ];

    let results = sjf(&mut processes);

    // Assert shortest jobs finish first
    assert_eq!(results[0].pid, processes[2].pid);
    assert_eq!(results[1].pid, processes[0].pid);
    assert_eq!(results[2].pid, processes[1].pid);

    // Add assertions for turnaround and waiting time
}
