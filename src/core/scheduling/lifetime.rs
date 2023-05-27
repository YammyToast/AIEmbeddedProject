use std::ptr::null;

use chrono::prelude::*;
use uuid::Uuid;

use crate::core::scheduling::{Schedulable, Executable};

pub struct LifetimeInstance {
    uuid: Uuid,
    priority: i8,
    bound_module: Box<dyn Schedulable>,
    spawn_timestamp: DateTime<Utc>,
    executables_spawned: u32
}

impl LifetimeInstance {
    // Not sure whether static is the correct call here!
    fn bound_module_spawn(mut self) -> Result<Vec<Box<Executable>>, ()> {
        let spawn_result = self.bound_module.spawn();
        match spawn_result {
            Err(e) => Err(e),
            Ok(executables) => {
                self.executables_spawned += executables.len() as u32;
                Ok(executables)
            }
        } 
    }

    fn bound_module_delete(self) -> bool {
        self.bound_module.delete_callback()
    }


    fn new(_module: Box<dyn Schedulable>, _priority: &i8) -> Box<LifetimeInstance> {
        Box::new(LifetimeInstance {
            uuid: Uuid::new_v4(),
            priority: *_priority,
            bound_module: _module, 
            spawn_timestamp: Utc::now(),
            executables_spawned: 0
        })
    }


}