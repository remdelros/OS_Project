use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
    pub id: String,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub priority: usize,
    pub remaining_time: usize,
}

// Implement Ord for Process
/*impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        self.remaining_time.cmp(&other.remaining_time)
            .then_with(|| self.priority.cmp(&other.priority))
            .then_with(|| self.arrival_time.cmp(&other.arrival_time))
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}*/

// Newtype wrapper for Process to implement Ord
#[derive(Clone, Debug)]
pub struct MyProcess(Rc<RefCell<Process>>);

impl Ord for MyProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.borrow().priority.cmp(&self.0.borrow().priority)
    }
}

impl PartialOrd for MyProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for MyProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = self.0.borrow();
        write!(f, "{} [{}-{}]", p.id, p.arrival_time, p.burst_time)
    }
}
