use self::executable::Executable;
use self::lifetime::LifetimeInstance;

pub mod executable;
pub mod lifetime;
pub mod scheduler;

pub trait Schedulable {
    fn spawn(&self, _lifetime: &LifetimeInstance) -> Result<Vec<Box<Executable>>, ()>;
    fn delete_callback(&self) -> bool;
    fn debug_string(&self) -> &str;
    fn execute(&self);
}
