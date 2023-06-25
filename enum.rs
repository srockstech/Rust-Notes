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

fn main(){
    which_way(Direction::Down);
    let a = Direction::Up;
    let _b = Direction::Left; // Underscore '_' is used at the start of the variable name to ignore the warning of unused variables
    let _c = Direction::Right;
    let _d = Direction::Forward;
    let _e = Direction::Backward;
    println!("{:?}", a);
    println!("{:?}", _b);
}