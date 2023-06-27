//Used to get a part of a string, an array or a vector
fn main() {
    let arr = [34,2,13,5,72,8];
    let stri = "Hello";
    let sli_arr = &arr[1..4];
    let sli_stri = &stri[1..4];
    println!("{:?}", sli_arr);
    println!("{:?}", sli_stri);

//By default, slices are immutable. But we can also make mutable slices using '&mut' for mutable collections.
    let mut mut_arr = [94,2,3,51,53];
    let mut_sli = &mut mut_arr[0..2];
    mut_sli[1] = 55;
    mut_sli[0] = 20;
    println!("{} {}", mut_sli[0], mut_sli[1]);
}