// Shared Ownership and Reference Counting
use std::thread;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{Cell, RefCell};

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
    // thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
    dbg!(a);

    // Naming Clones
    let n = Arc::new([1, 3, 5]);
    thread::spawn({
        let n = n.clone();
        move || {
            dbg!(n);
        }
    });
    dbg!(n);
    // n.sort(); // Arc doesn't give mutable access to their contained value

    // Undefined Behaviour
    let a = [123, 456, 789];
    unsafe {
        let b = a.get_unchecked(2); //  Causes an Undefined Behaviour as index of 3 is not on the list
        println!("{b}")
    }

    // Interior Mutability
    let a = Cell::new(5);
    let b = &a;
    let c = &a;
    f_2(b, c);

    let a = Cell::new(vec![1, 2, 3]);
    f_3(&a);

    // Thread Safety: Send and Sync
    // - Send and Sync are special traits that if when implemented tells the compiler that
    //   a type is safe to use accross threads. In other words, if a type can be used across threads
    //   then it implements this traits. 
    // - A type is Send if it can be sent to another thread. That is, if ownership of a value of that type
    //   can be tranferred to another thread.
    // - A type is Sync if it allows a shared reference accross threads. That is, there is a guarantee that they won't be a
    //   case where they are trying to mutate the same value. The likes i32, bool and other primitive types gives this guarantee
    //   because there references can't be mutated and the likes Mutex handles synchronization properly making sure two mutations
    //   aren't happening simultaneously. Cell and RefCell on the other other handle doesn't give that guarantee (notice how the function f_2 works) 
    //   hence they are Send but not async.
}

fn x() {
    println!("Not same");
}

fn f_1(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x();
    }
}

fn f_2(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x();
    }
}

fn f_3(v: &Cell<Vec<i32>>) {
    // - Cell doesn't allow us to borrow its content rather we have to `take` it and then replace it later.
    // - Cell can only be used in a single thread
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

fn f_4(v: &RefCell<Vec<i32>>) {
    // - As shown below, RefCell can be borrowed and mutated unless Cell where we had to totally replace the value
    // - But, similar to Cell, this can also only be used in a single thread
     v.borrow_mut().push(1); // We can modify the `Vec` directly.
}