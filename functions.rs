fn subtract(a: i32, b:i32) -> i32 {
    a - b
}

fn main() {
    let a = 10;
    let b = 100;
    let c = subtract(a, b);
    println!("{}", c);
}