use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use uuid::Uuid;

use crate::core::scheduling::LifetimeInstance;

use super::executable::Executable;

pub fn init_lifetime_vec() -> Box<HashMap<Uuid, Box<LifetimeInstance>>> {
    // After each iteration, lifetimes are called to determine whether they want to add to the queue.
    // Vector of lifetimes current alive in this context.
    // Each lifetime is queried during each iteration of the scheduling process to create program functionality.
    let lifetimes: Box<HashMap<Uuid, Box<LifetimeInstance>>> = Box::new(HashMap::new());
    lifetimes
}

pub fn init_queue() -> Rc<VecDeque<Box<Executable>>> {
    // Reference Counted Double-ended Queue for executing lifetime (module) functions.
    // Main Execution Queue.
    let schedule_queue: Rc<VecDeque<Box<Executable>>> = Rc::new(VecDeque::new());
    schedule_queue
}

pub fn reschedule_queue(
    _lifetimes: Rc<VecDeque<Box<i32>>>,
    _schedule_queue: &mut Rc<VecDeque<Box<i32>>>,
) {
    let test = Rc::get_mut(_schedule_queue);
    todo!()
}
