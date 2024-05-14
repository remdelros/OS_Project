use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::option::Option;
use core::option::Option::Some;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    pub id: String,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub priority: usize,  // Keep priority for priority scheduling
    pub remaining_time: usize,
}

// Implement Ord based on remaining_time first, then priority, then arrival_time
impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        self.remaining_time.cmp(&other.remaining_time)  // SRTF comparison
            .then_with(|| self.priority.cmp(&other.priority))  // Priority comparison (if remaining times are equal)
            .then_with(|| self.arrival_time.cmp(&other.arrival_time)) // Arrival time comparison (if priorities are also equal)
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

