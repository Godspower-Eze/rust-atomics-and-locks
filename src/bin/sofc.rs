// Shared Ownership and Reference Counting
use std::thread;

 static X: [i32; 3] = [1, 2, 3];

fn main() {
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}
