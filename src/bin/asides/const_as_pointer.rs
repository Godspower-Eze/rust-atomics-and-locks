static S: i32 = 5;

fn main() {
    let ptr = &S as *const i32;
    let ptr_1 = &S;

    println!("{:?}", ptr);
    println!("{:p}", ptr);
    println!("{:p}", ptr_1);

    let p: *const i32 = std::ptr::null();
    // Deferencing of a null raw pointer
    // unsafe {
    //      println!("{:?}", *p);
    // }
    let x = 5;
    let p = &x as *const i32;
    unsafe {
        println!("{}", *p);
    }
}