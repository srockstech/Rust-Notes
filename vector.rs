fn main(){
    let mut numbers: Vec<i32> = Vec::new(); //Declaring and initializing an empty vector
    let mut nums = Vec::new(); //Method 2 for declaring and initializing an empty vector, but this will give error if no element is pushed into it because the compiler won't know it's explicit type.
    //One an element is pushed, the type of the element pushed will become the type of the vector.

    numbers.push(32);
    numbers.push(23);
    numbers.push(53);
    numbers.push(75);
    nums.push(1);
    println!("{:?}", numbers);
    println!("{:?}", nums);
    println!("{}", numbers[2]);
    println!("{}", nums[0]);

    //Iterating over a vector
    for num in numbers {
        println!("{}", num);
    }
}