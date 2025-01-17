use std::thread::sleep;
use std::time::Duration;

// does nothing and wastes time
fn main() {
    loop {
        println!("i'm stupid");
        sleep(Duration::from_nanos(10))
    }
}
