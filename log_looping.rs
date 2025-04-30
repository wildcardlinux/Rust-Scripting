use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut x = 0;
    while x < 25 {
        println!("This is a loop...");
        sleep(Duration::from_millis(1000));
        //if x == 3 { continue } // skipping iteration when x is equal to 3.
        x += 1;
}
}
