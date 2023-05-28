use std::collections::{HashMap, VecDeque};
use std::ptr::null;
use std::rc::Rc;
use uuid::Uuid;

use crate::core::scheduling::LifetimeInstance;

use super::executable::Executable;

pub struct LifetimeMap {
    map: Box<HashMap<Uuid, Box<LifetimeInstance>>>,
}

impl LifetimeMap {
    pub fn new() -> LifetimeMap {
        LifetimeMap {
            map: Box::new(HashMap::new())
        }
    }
}

pub struct ExecutablesQueue {
    que: Rc<VecDeque<Box<Executable>>>,
}

impl ExecutablesQueue {
    pub fn new() -> ExecutablesQueue {
        ExecutablesQueue {
            que: Rc::new(VecDeque::new()) 
        }
    }

    pub fn reschedule_queue(self) {
        

    }

    pub fn execute_queue(self) {
        loop {
            let front: &Box<Executable>;
            match self.que.front() {
                None => break,
                Some(e) => {
                    front = e;        
                }
            } 
            // Get pointer to execution function of the current executable.
            let front_execution_function = *(**front).execution_function;
            // Call pointer to execution function.
            front_execution_function()
        }

    }

}


pub fn reschedule_queue(
    _lifetimes: LifetimeMap,
    _schedule_queue: &mut ExecutablesQueue,
) {
    let test = Rc::get_mut(&mut _schedule_queue.que);
    todo!()
}
