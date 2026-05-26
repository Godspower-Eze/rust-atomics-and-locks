// Shared Ownership and Reference Counting
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    // thread::spawn(move || dbg!(b));
    // dbg!(a);

    // Naming Clones
    // let n = Arc::new([1, 3, 5]);
    // thread::spawn({
    //     let n = n.clone();
    //     move || {
    //         dbg!(n);
    //     }
    // });
    // dbg!(n);
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
    // - Send and Sync are special traits that when implemented tells the compiler that
    //   a type is safe to use across threads. In other words, if a type can be used across threads
    //   then it implements this traits.
    // - A type is Send if it can be sent to another thread. That is, if ownership of a value of that type
    //   can be tranferred to another thread.
    // - A type is Sync if it allows a shared reference accross threads. That is, there is a guarantee that they won't be a
    //   case where multiple threads are trying to mutate the same value. The likes of i32, str, and bool
    //   gives this guarantee because there references can't be mutated and the likes of Mutex handles synchronization properly
    //   making sure two mutations aren't happening simultaneously. Cell and RefCell on the other
    //   hand doesn't give that guarantees (notice how the function f_2 works) hence they are Send but not async.
    // - Both of these traits are auto traits which means they are automatically implemented for your types baseed on their
    //   fields. A struct with fields that are all Send and Sync it itself also Send and Sync
    struct X {
        handle: i32,
        _not_sync: PhantomData<Cell<()>>,
    }
    // In this case X would have being automatically "inherited" the Send and Sync from handle being of i32 type but the second field blocks that because
    // Cell doesn't implement them.
    let a = Rc::new(123);
    // The code below would throw an error as RC dooesn't implement the Send trait
    // thread::spawn(move || {
    //     dbg!(a);
    // });

    // Locking: Mutexes and RwLocks
    // - Mutex is short for mutual exclusion
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);

    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                thread::sleep(Duration::from_secs(1));
            });
        }
        // A Mutex works by making access to the value it protects exclusive to
        // a single thread at every point in time and when the value is dropped
        // the next thread picks up the value. That is, they are now able to lock
        // and use the value.
        //
        // The above case shows how before dropping(out of scope), we add a one second
        // delay before going out of scope. This ultimately shows the idea of
        // how and when other threads gets access to a value in a Mutex.
        //
        // This would run in about ten seconds as every thread sleeps for one. But then, with
        // this we don't get the value of threading. Let look at the next one below.
    });
    assert_eq!(n.into_inner().unwrap(), 1000);

    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    println!("{}", guard);
                    *guard += 1;
                }
                drop(guard); // New: drop the guard before sleeping!
                thread::sleep(Duration::from_secs(1));
            });
        }
        // Here we are explicitly dropping the guard before sleeping which gives
        // access to the next thread before sleeping making this run in
        // about a second. This is what parrellelism looks like and that's one
        // of the benefit of threading.
        //
        // "This shows the importance of keeping the amount of time a mutex is locked as short as possible.
        // Keeping a mutex locked longer than necessary can completely nullify any benefits of parallelism,
        // effectively forcing everything to happen serially instead"
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
    
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                // panic!()
            });
        }
        // What happens when a thread panics while holding a lock?
        // 
        // When a thread holding a lock panics, the lock is considered poisoned and
        // when a lock is poisoned, the Mutex will no longer be locked.
        // And then, calling n.lock() actually locks the Mutex and returns an Err containing the value.
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
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
