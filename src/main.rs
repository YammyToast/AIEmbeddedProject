use crate::core::scheduling::scheduler::{LifetimeMap, ExecutablesQueue};

mod core;

fn main() {
    let lifetimes = LifetimeMap::new();
    let schedule = ExecutablesQueue::new();
}
