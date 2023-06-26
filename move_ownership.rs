//Valid only for 'struct' and 'enum' because they don't implement copy trait.
//Owner of the data must clean up the memory. This occurs automatically at the end of the scope.
//When a 'struct' or 'enum' value is passed to a function, by default, the value moves to the called function.
//After the function call ends, the moved value gets deleted from the calling function as well.
//This prevents from reusing the value in the calling function.
//If we don't want the value to be deleted, we use '&' symbol with both actual and former parameters.

struct Book{
    pages: i32,
    rating: i32
}

fn display_page_count(book: Book){ //'&' symbol at both actual and formal parameters ensures the value isn't moved to the called function body.
    println!("pages = {}", book.pages);
}

fn display_rating(book: Book){
    println!("rating = {}", book.rating);
}

fn main(){
    let book1 = Book{
        pages: 300,
        rating: 8
    };

    display_page_count(book1); //In this pass by value, copy is not made in the called function, rather ownership of the variable is given to the called function. (occurs only in case of a struct or enum)
    println!("{}", book1.pages); //This gives errors as 'book1' has been deleted when the called function ended.

}