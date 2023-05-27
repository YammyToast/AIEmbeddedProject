use std::collections::VecDeque;
use std::rc::Rc;

use crate::core::scheduling::lifetime;

pub fn init_lifetime_vec() -> Box<Vec<Box<i32>>> {
    // After each iteration, lifetimes are called to determine whether they want to add to the queue.
    // Vector of lifetimes current alive in this context.
    // Each lifetime is queried during each iteration of the scheduling process to create program functionality.
    let lifetimes: Box<Vec<Box<i32>>> = Box::new(vec![]);
    return lifetimes;
}

pub fn init_queue() -> Rc<VecDeque<Box<i32>>> {
    // Reference Counted Double-ended Queue for executing lifetime (module) functions.
    // Main Execution Queue.
    let schedule_queue: Rc<VecDeque<Box<i32>>> = Rc::new(VecDeque::new());
    schedule_queue
}

pub fn reschedule_queue(
    _lifetimes: Rc<VecDeque<Box<i32>>>,
    _schedule_queue: &mut Rc<VecDeque<Box<i32>>>,
) {
    let test = Rc::get_mut(_schedule_queue);
    todo!()
}
