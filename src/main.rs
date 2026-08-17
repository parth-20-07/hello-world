use core::time;
use std::thread::sleep;

fn main() {
    loop {
        println!("Hello World!");
        sleep(time::Duration::from_secs(2));
    }
}
