mod core;

fn main() {
    let lifetimes = core::scheduling::scheduler::init_lifetime_vec();
    // println!("{lifetimes:?}");
    let schedule = core::scheduling::scheduler::init_queue();
    
}
