//same as switch statement
fn main(){
    let a = "banana";
    match a {
        "banana" => println!("It is a banana"),
        "apple" => println!("It is an apple"),
        _ => println!("It is something else"),
    }
}