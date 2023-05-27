
use uuid::Uuid;

pub struct Executable<'a> {
    lifetime_uuid: Uuid,
    execution_function: &'a fn(),
    

}