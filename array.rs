fn main() {
    let arr1: [i32; 4] = [1,23,4,5]; // Method 1 to declare and initialize an array
    let arr2 = [9.0, 20.0, 80.9]; // Methood 2 to declare and initialze and array
    let arr3: [&str; 3] = ["Hello", "Okay", "Right"];
    println!("{} {}", arr1[0], arr1[3]);
    println!("{} {}", arr2[0], arr2[2]);
    println!("{} {}", arr3[1], arr3[2]);
    println!("{}", arr1.len()); //To get the length of an array

    //Array iteration
    for num in arr1.iter(){
        println!("{}", num);
    }
}