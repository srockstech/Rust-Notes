// If_else and match can be used in assignment expression.
fn main(){

    //if_else
    let num = 1000;
    let is_single_digit = if num < 10 {
        true
    } else {
        false
    };
    println!("{}", is_single_digit);

    //match
    let num1 = 8;
    let bool_val = match num1 {
        0 => false,
        _ => true
    };
    println!("{}", bool_val);
}