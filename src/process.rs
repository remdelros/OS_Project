use core::cmp::PartialEq;
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


// Newtype wrapper for Process to implement Ord
#[derive(Clone, Debug)]
#[derive(Eq)]
pub struct MyProcess(pub(crate) Rc<RefCell<Process>>);

impl PartialEq for MyProcess {
    fn eq(&self, other: &Self) -> bool {
        // Logic to determine if the wrapped Process instances are equal
        self.0.borrow().id == other.0.borrow().id  // Compare by process ID, for example
    }
}

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
