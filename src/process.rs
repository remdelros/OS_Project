use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use core::option::Option;
use core::option::Option::Some;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    pub id: String,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub priority: usize,
    pub remaining_time: usize,
}

pub struct MyProcess(Process);


impl Eq for MyProcess {}

impl PartialEq<Self> for MyProcess {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Ord for MyProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.priority.cmp(&self.0.priority)
    }
}


impl PartialOrd for MyProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


