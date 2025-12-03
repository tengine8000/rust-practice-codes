/*
    fn add_nums_f(x: i32, y: i32) -> i32 {
        x + y
    }
    let add_nums_c = |x, y| x + y;

    let a = add_nums_f(10, 15);
    println!("a = {}", a);
    let b = add_nums_c(10, 15);
    println!("b = {}", b);
    let c = add_nums_c(10.5, 12.5) // This will cause a compile-time error as the closure expects integers not floats.
    println!("c = {}", c);

    Example 2
    struct Circle<T>
where
    T: Fn(f32) -> f32,
{
    radius: f32,
    area: T,
}
fn main() {
    let pi = 3.14;
    let area_circle = |r| pi * r * r;
    let c = Circle {
        area: area_circle,
        radius: 10.0,
    };
    println!("Area of circle: {}", (c.area)(c.radius));
}
 Example 3
     let num = 5;
    let add_num = |x| x + num;
    let a = add_num(10);
    println!("a = {}", a);

    Example 4
        let arr = [1, 2, 3, 4, 5];
    let mut arr_iter = arr.iter();
    // for val in arr_iter {
    //     println!("val = {}", val);
    // }
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    // println!("{}", arr_iter.next().unwrap()); // panic due to unwrap() on None value

    Example 5
    struct CounterFinite {
    count: u32,
}

impl Iterator for CounterFinite {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct CounterInfinite {
    count: u32,
}

impl Iterator for CounterInfinite {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

    fn main() {
    let mut cf = CounterFinite { count: 0 };
    // for i in cf {
    //     println!("i = {}", i);
    // }
    println!("{}", cf.next().unwrap());
    println!("{}", cf.next().unwrap());

    let mut ci = CounterInfinite { count: 0 };
    println!("{}", ci.next().unwrap());
    println!("{}", ci.next().unwrap());
}

    Example 6
        let mut v = vec![1, 2, 3, 4, 5];
    let iter = v.iter_mut(); // exploring iter(), into_iter(), iter_mut()
    for val in iter {
        *val *= 100;
    }
    println!("{:?}", v); // reusing the collection

    Example 7: Performance comparison between for loop and iterator
    use std::time::Instant;
// sum using for loop
fn sum_1(x: &[i64]) -> i64 {
    let mut result: i64 = 0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}
// sum using iterator
fn sum_2(x: &[i64]) -> i64 {
    x.iter().sum()
}

fn main() {
    let mut v = vec![0; 1000000];
    let mut count = 1;
    for i in 0..v.len() {
        v[i] = count;
        count += 1;
    }
    for i in 0..10 {
        println!("Run {} : ", i);
        let mut now = Instant::now();
        let sum1 = sum_1(&v);
        println!("sum_1: {} / {} ms", sum1, now.elapsed().as_millis());
        now = Instant::now();
        let sum2 = sum_2(&v);
        println!("sum_2: {} / {} ms", sum2, now.elapsed().as_millis());
    }
}
    Example 8: Custom Iterator that yields multiples of 3
    struct ThreeCounter {
    count: u32,
}

impl Iterator for ThreeCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 3;
        Some(self.count)
    }
}

fn main() {
    let mut tc = ThreeCounter { count: 0 };
    for _ in 0..5 {
        println!("tc = {}", tc.next().unwrap());
    }
}

struct ThreeCounter {
    count: u32,
}

impl Iterator for ThreeCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 3;
        Some(self.count)
    }
}

fn main() {
    let mut tc = ThreeCounter { count: 0 };
    for _ in 0..5 {
        println!("tc = {}", tc.next().unwrap());
    }
}

*/
