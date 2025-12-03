use std::ops::Mul;

#[derive(Debug)]
struct Circle<T> {
    cx: T,
    cy: T,
    r: T,
}
impl<T> Circle<T> {
    fn radius(&self) -> &T {
        &self.r
    }
}
// Specialized implementation for f64 type
// impl Circle<f64> {
//     fn radius(&self) -> &f64 {
//         &self.r
//     }
// }

fn main() {
    let a = 10;
    let b = 20;
    let c = sum_int(a, b);
    println!("c = {}", c);

    let d = 10.5;
    let e = 20.5;
    let f = sum_float(d, e);
    println!("f = {}", f);

    let c1 = Circle {
        cx: 10,
        cy: 20,
        r: 5,
    };
    println!("Circle,c1 = {:?}", c1);
    // println!("Circle c1 radius = {:?}", c1.radius());

    let c2 = Circle {
        cx: 10.5,
        cy: 20.5,
        r: 5.5,
    };
    println!("Circle,c2 = {:?}", c2);
    println!("Circle c2 radius = {:?}", c2.radius());

    let c3 = Circle {
        cx: "x_center",
        cy: "y_center",
        r: "origin_radius",
    };
    println!("Circle,c3 = {:?}", c3);
    println!("Circle c3 radius = {:?}", c3.r);

    let length_int = 10;
    let width_int = 20;
    let area_int = area_rect(length_int, width_int);
    println!(
        "Area of rectangle with length {} and width {} is {}",
        length_int, width_int, area_int
    );

    let length_float = 10.5;
    let width_float = 20.5;
    let area_float = area_rect(length_float, width_float);
    println!(
        "Area of rectangle with length {} and width {} is {}",
        length_float, width_float, area_float
    );

    generic_example();
}
fn sum_int(x: i32, y: i32) -> i32 {
    x + y
}
fn sum_float(x: f64, y: f64) -> f64 {
    x + y
}
fn area_rect<T: Mul<Output = T>>(length: T, width: T) -> T {
    length * width
}

// my custom enum with generics
enum Processor<I, P, O> {
    Input(I),
    Process(P),
    Output(O),
}

impl<I, P, O> Processor<I, P, O> {
    fn describe(&self) {
        match self {
            Processor::Input(_) => println!("This is an Input processor"),
            Processor::Process(_) => println!("This is a Process processor"),
            Processor::Output(_) => println!("This is an Output processor"),
        }
    }
}

fn generic_example() {
    let input_processor: Processor<&str, &str, &str> = Processor::Input("InputData");
    let process_processor: Processor<&str, &str, &str> = Processor::Process("ProcessData");
    let output_processor: Processor<&str, &str, &str> = Processor::Output("OutputData");

    input_processor.describe();
    process_processor.describe();
    output_processor.describe();

    println!("---------------------------");
    println!("Start from input processor");
    if let Processor::Input(data) = input_processor {
        println!("Processing input data: {}", data);
    }
    println!("---------------------------");
    if let Processor::Process(data) = process_processor {
        println!("Processing data: {}", data);
    }
    println!("---------------------------");
    if let Processor::Output(data) = output_processor {
        println!("Output data: {}", data);
    }
}
