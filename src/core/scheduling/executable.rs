
use uuid::Uuid;

pub struct Executable {
    lifetime_uuid: Uuid,
    execution_function: &'static fn(),
    

}