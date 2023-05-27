use std::str::FromStr;
use crate::core::scheduling::{lifetime::LifetimeInstance, Schedulable, executable::Executable};
use chrono::prelude::*;
use std::path::Path;


pub struct Logger {
    output_file: Box<Path>,
    buffer: String

}

impl Logger {
    fn print_buffer() {


    }    

}

impl Schedulable for Logger {
    fn spawn(&self) -> Result<Vec<Box<Executable>>, ()> {
        let mut executable_buffer: Vec<Executable> = Vec::new();
        executable_buffer.push(Executable::new());
        Err(())
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