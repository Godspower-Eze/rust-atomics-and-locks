use std::thread;
use rust_atomics_and_locks::f;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3];
    let numbers_1 = vec![4, 5, 6];

    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
        println!("{:?}", numbers_1);
    }).join().unwrap();

    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");

    // Scoped Threads
    let nums = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", nums.len());
        });
        s.spawn(|| {
            for n in &nums {
                println!("{n}");
            }
        });
    });

    let mut nums = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            nums.push(1);
        });
        // s.spawn(|| {
        //     nums.push(2);
        // });
    });
}
