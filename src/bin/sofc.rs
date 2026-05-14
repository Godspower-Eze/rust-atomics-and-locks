// Shared Ownership and Reference Counting
use std::thread;
use std::rc::Rc;
use std::sync::Arc;

static X: [i32; 3] = [1, 2, 3];

fn main() {
    // thread::spawn(|| dbg!(&X));
    // thread::spawn(|| dbg!(&X));

    // Leaking
    // let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    // thread::spawn(move || dbg!(x));
    // thread::spawn(move || dbg!(x));

    // Reference Counting
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
    // thread::spawn(move || dbg!(b)); 
    // This will not compile as RC (Reference Counted) is not thread safe. 
    // Rather, we will use Arc (Atomically Reference Counted)
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}
