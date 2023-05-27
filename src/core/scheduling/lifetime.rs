use chrono::prelude::*;
use uuid::Uuid;

struct Lifetime {
    uuid: Uuid,
    priority: u8,
    bound_module: i32,
    spawn_timestamp: DateTime<Utc>
}

impl Lifetime {
    fn bound_module_spawn() -> Result<Vec<i32>, ()> {

        Err(())
    }

    fn bound_module_delete() -> bool {
        false

    }

}