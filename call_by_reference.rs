//Call by reference
fn changevalue(x: &mut i32){
    *x = 11;
}

fn main(){
    let mut a = 9;
    changevalue(&mut a);
    println!("{}", a);
}