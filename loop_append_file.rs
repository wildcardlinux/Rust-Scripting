use std::time::Duration;
use std::thread::sleep;
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() {
    let mut x = 0;
    let mut file = OpenOptions::new()
        .append(true)
        .open("foo.txt")
        .unwrap();
    while x < 25 {
        writeln!(file, "This is looping text...");
        sleep(Duration::from_millis(1000));
        //if x == 3 { continue } // skipping iteration when x is equal to 3.
        x += 1;
}
}
