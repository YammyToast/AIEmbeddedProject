use self::lifetime::LifetimeInstance;
use self::executable::Executable;

pub mod scheduler;
pub mod lifetime;
pub mod executable;

pub trait Schedulable {
    fn spawn(&self) -> Result<Vec<Box<Executable>>, ()>;
    fn delete_callback(&self) -> bool;
    fn debug_string(&self) -> &str;
    fn execute(&self);
}
