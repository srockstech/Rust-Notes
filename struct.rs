struct GroceryItem {
    stock: i32,
    price: f64
}

fn main(){
    let cereal = GroceryItem{
        stock: 10,
        price: 2.99
    };

    println!("Available Stock for cereal: {}", cereal.stock);
    println!("Unit price of cereal: {}", cereal.price);
}