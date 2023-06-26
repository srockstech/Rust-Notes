//Owner of the data must clean up the memory. This occurs automatically at the end of the scope.
//When a value is passed to a function, by default, the value moves to the called function.
//After the function call ends, the moved value gets deleted from the calling function as well.
//This prevents from reusing the value in the calling function.
//If we don't want the value to be deleted, we use '&' symbol with both actual and former parameters.

struct Book{
    pages: i32,
    rating: i32
}

fn display_page_count(book: Book){ //'&' symbol at both actual and formal parameters ensures the value isn't moved to the called function body
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

    display_page_count(book1);
    display_rating(book1);

}