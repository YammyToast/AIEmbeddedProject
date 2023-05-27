use uuid::Uuid;

pub struct Executable {
    lifetime_uuid: Uuid,
    execution_function: Box<fn()>,
    queue_position: usize,
}

impl Executable {
    pub fn new() -> Box<Executable> {
        Box::new(Executable {
            lifetime_uuid: Uuid,
            execution_function: (),
            queue_position: (),
        })
    }
}
