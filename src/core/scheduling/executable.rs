use uuid::Uuid;

pub struct Executable {
    lifetime_uuid: Uuid,
    pub execution_function: Box<fn()>,
    queue_position: usize,
}

impl Executable {
    pub fn new(_execution_function: Box<fn()>) -> Box<Executable> {
        Box::new(Executable {
            lifetime_uuid: Uuid::new_v4(),
            execution_function: _execution_function,
            queue_position: 0,
        })
    }
}
