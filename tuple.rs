fn main(){
    let mut a = ("hello", 30, 90.5);
    println!("{} {} {}", a.0, a.1, a.2);

    let (mut b, mut c, mut d) = ("Hi", 44, 99);
    println!("{} {} {}", b, c, d); 

    a.2 = 93.2;
    b = "Hello";
    c = 50;
    d = 0;

    let mut x = ("Ram", 31); //A tuple can have 2 or more values of same or different types

    println!("{} {} {} {}", a.2, b, c, d);
    println!("{} {}", x.0, x.1);
}