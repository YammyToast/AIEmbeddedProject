pub mod scheduler;
pub mod lifetime;

pub trait Schedulable {
    fn spawn() -> Result<Vec<i32>, ()>;
    fn delete_callback() -> bool;
    fn log_string() -> str;
}
