use std::str::FromStr;
use crate::core::scheduling::{lifetime::LifetimeInstance, Schedulable, executable::Executable};



struct Logger {


}

impl Schedulable for Logger {
    fn spawn(&self) -> Result<Vec<Executable>, ()> {

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