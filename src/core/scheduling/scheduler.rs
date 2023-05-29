use std::collections::{HashMap, VecDeque};
use std::ptr::null;
use std::rc::Rc;
use uuid::Uuid;

use crate::core::scheduling::LifetimeInstance;

use super::executable::Executable;
use super::lifetime;

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
    que: Box<VecDeque<Box<Executable>>>,
}

impl ExecutablesQueue {
    pub fn new() -> ExecutablesQueue {
        ExecutablesQueue {
            que: Box::new(VecDeque::new()) 
        }
    }

    pub fn reschedule_queue(mut self, _lifetimes: LifetimeMap) {
        (*self.que).clear();     
        for lifetime in _lifetimes.map.values() {


        }

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
