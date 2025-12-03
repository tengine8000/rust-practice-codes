    // Example demonstrating heap allocation using Box in Rust.
fn main() {
    let x = 10;
    let y = 20;
    let x_p = &x;
    println!("x = {}, &x = {:p}", x, &x);
    println!("y = {}, &y = {:p}", y, &y);
    println!("x_p = {:p}", x_p);
}
    // Example demonstrating heap allocation using Box in Rust.
fn main() {
    let x = Box::new(10.5);
    println!("x = {}", x);

    // Example demonstrating recursive data structures using Box.
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

    // Example demonstrating reference counting using Rc.
#[derive(Debug)]
enum A {
    B(i32),
    C(i32, Rc<A>),
}
use std::rc::Rc;
fn main() {
    let x = Rc::new(A::C(1, Rc::new(A::C(2, Rc::new(A::B(3))))));
    {
        println!("count of x = {}", Rc::strong_count(&x));
        let _y = A::C(5, Rc::clone(&x));
        println!("count of x = {}", Rc::strong_count(&x));
        {
            let _z = A::C(10, Rc::clone(&x));
            println!("count of x = {}", Rc::strong_count(&x));
        }
        println!("count of x = {}", Rc::strong_count(&x));
    }
    println!("count of x = {}", Rc::strong_count(&x));
}

    // Example demonstrating interior mutability using RefCell.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum A {
    B(i32),
    C(Rc<RefCell<i32>>, Rc<A>),
}

fn main() {
    let x = Rc::new(RefCell::new(10));
    let y = Rc::new(A::C(Rc::clone(&x), Rc::new(A::B(1))));
    println!("y = {:?}", y);
    *x.borrow_mut() += 100;
    println!("y mutated = {:?}", y);
}

