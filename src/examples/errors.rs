use std::num::ParseIntError;

fn main() {
    let side = "52";
    let area = area_square(side);
    println!("Area of square with side {} is {}", side, area);
    let x = -10;
    if x < 0 {
        panic!("x cannot be negative: {}", x);
    }
    println!("x is {}", x);
}

fn area_square(side: &str) -> i32 {
    let x: Result<i32, ParseIntError> = side.parse();
    let x = match x {
        Ok(l) => l,
        Err(e) => panic!("Error occurred: {}", e),
    };
    x * x
}
