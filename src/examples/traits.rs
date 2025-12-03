fn main() {
    let shape1 = Circle {
        cx: 10,
        cy: 20,
        r: 5.0,
    };
    // shape1.print_shape();
    // println!("Area of circle {}", shape1.area());
    // println!("Perimeter of circle {}", shape1.perimeter());
    // draw_shape(&shape1);

    let shape2 = Rectangle {
        length: 10.0,
        width: 20.0,
    };
    // shape2.print_shape();
    // println!("Area of rectangle {}", shape2.area());
    // println!("Perimeter of rectangle {}", shape2.perimeter());
    // draw_shape(&shape2);

    draw_shapes(&shape1, &shape2);
    draw_flying_shapes(&shape1);
    draw_flying_shapes(&shape2);

    let vec1 = vec![10.02, 3.43, 4.5, 8.9, 9.02];
    println!("vec1 = {:?}", &vec1);
    println!("Max of vec1 = {:?}", find_max(&vec1));
}

trait ShapeUtils {
    fn print_shape(&self) {
        println!("This is a 2D shape.");
    }
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait Flyable {
    fn fly(&self) {
        println!("This object can fly.");
    }
}

struct Circle {
    cx: i32,
    cy: i32,
    r: f64,
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl ShapeUtils for Circle {
    fn print_shape(&self) {
        println!(
            "Circle: center = ({}, {}), radius = {}",
            self.cx, self.cy, self.r
        );
    }
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.14 * self.r
    }
}

impl ShapeUtils for Rectangle {
    fn print_shape(&self) {
        println!(
            "Rectangle: length = {}, width = {}",
            self.length, self.width
        );
    }
    fn area(&self) -> f64 {
        self.length * self.width
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}

impl Flyable for Circle {
    fn fly(&self) {
        println!("Circle can fly like a frisbee!");
    }
}

impl Flyable for Rectangle {
    fn fly(&self) {
        println!("Rectangle can fly like a paper airplane!");
    }
}
// fn draw_shape(shape: &impl ShapeUtils) {
//     shape.print_shape();
// }

fn draw_shape<T: ShapeUtils>(shape: &T) {
    shape.print_shape();
}

fn draw_shapes(shape1: &impl ShapeUtils, shape2: &impl ShapeUtils) {
    shape1.print_shape();
    shape2.print_shape();
}

// fn draw_shapes(shape1: &Circle, shape2: &Rectangle) {
//     shape1.print_shape();
//     shape2.print_shape();
// }

fn draw_flying_shapes(shape: &(impl ShapeUtils + Flyable)) {
    shape.print_shape();
    shape.fly();
}

fn find_max(vectors: &Vec<f64>) -> Option<&f64> {
    if vectors.is_empty() {
        return None;
    }
    let mut max = &vectors[0];
    for val in vectors.iter() {
        if val > max {
            max = val;
        }
    }
    Some(max)
}
