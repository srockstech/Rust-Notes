fn which_way(go: Direction){
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
        Direction::Forward => println!("forward"),
        _ => println!("Some other way")
    }
}

#[derive(Debug)] //enums cannot be printed. So in order to print 
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward
}

//Enums with data: This allows to attach additional values to enum values
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

fn print_area(shape: Shape){
    match shape {
        Shape::Circle(radius) => {
            let area = std::f64::consts::PI * radius * radius;
            println!("Area of the circle: {}", area);
        },
        Shape::Rectangle(length, breadth) => {
            let area = length * breadth;
            println!("Area of the rectangle: {}", area);
        },
        Shape::Triangle(side1, side2, side3) => {
            let s = (side1 + side2 + side3)/2.0;
            let area = (s * (s - side1) * (s - side2) * (s - side3)).sqrt();
            println!("Area of the triangle: {}", area);
        }
    }
}

fn main(){
    which_way(Direction::Down);
    let a = Direction::Up;
    let _b = Direction::Left; // Underscore '_' is used at the start of the variable name to ignore the warning of unused variables
    let _c = Direction::Right;
    let _d = Direction::Forward;
    let _e = Direction::Backward;
    println!("{:?}", a);
    println!("{:?}", _b);

    print_area(Shape::Triangle(60.0, 60.0, 50.0));
}