//Call by value
fn changevalue(mut x: i32){
    x = 11;
    println!("{}", x);
}

fn main(){
    let mut a = 9;
    changevalue(a);
    println!("{}", a);
}