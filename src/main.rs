/*
Example demonstrating heap allocation using Box in Rust.
fn main() {
    let x = 10;
    let y = 20;
    let x_p = &x;
    println!("x = {}, &x = {:p}", x, &x);
    println!("y = {}, &y = {:p}", y, &y);
    println!("x_p = {:p}", x_p);
}
    Example demonstrating heap allocation using Box in Rust.
fn main() {
    let x = Box::new(10.5);
    println!("x = {}", x);
}


*/
#[derive(Debug)]
enum A {
    B(i32),
    C(i32, Box<A>),
}
fn main() {
    let a = A::C(1, Box::new(A::C(2, Box::new(A::B(3)))));
    let sz = std::mem::size_of_val(&a);
    println!("a created {:?}", a);
    println!("size of a = {}", sz);
}
