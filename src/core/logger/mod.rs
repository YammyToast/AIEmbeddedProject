use crate::core::scheduling::{executable::Executable, lifetime::LifetimeInstance, Schedulable};
use chrono::prelude::*;
use std::path::Path;
use std::str::FromStr;

pub struct Logger {
    output_file: Box<Path>,
    buffer: String,
}

impl Logger {
    fn print_buffer() {
        println!()
    }
}

impl Schedulable for Logger {
    fn spawn(&self, _lifetime: &LifetimeInstance) -> Result<Vec<Box<Executable>>, ()> {
        let mut executable_buffer: Vec<Box<Executable>> = Vec::new();
        executable_buffer.push(Executable::new(Box::new(Logger::print_buffer)));
        Ok(executable_buffer)
    }

    fn delete_callback(&self) -> bool {
        todo!()
    }

    fn debug_string(&self) -> &str {
        todo!();
        "temp"
    }

    fn execute(&self) {
        todo!()
    }
}
