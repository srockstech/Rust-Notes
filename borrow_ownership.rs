//Valid only for struct and enum because they don't implement copy trait.
struct Book{
    pages: i32,
    rating: i32
}

fn display_page_count(book: &Book){ //'&' symbol at both actual and formal parameters ensures the value isn't moved to the called function body.
    println!("pages = {}", book.pages);
}

fn display_rating(book: &Book){
    println!("rating = {}", book.rating);
}

fn main(){
    let book1 = Book{
        pages: 300,
        rating: 8
    };

    display_page_count(&book1); //In this pass by reference, just the reference (address) has been passed.
    display_rating(&book1);
    println!("{}", book1.pages); //This doesn't give error as 'book1' has been passed by reference, that means called function just borrows the ownership.
    println!("{}", book1.rating);
}