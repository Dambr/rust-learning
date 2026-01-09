mod book;

use rand::Rng;
use crate::book::IBook;

fn main() {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(1..100);
    println!("random number: {}", random_number);

    let book = book::Book::new("Title", "Author", 32, 24);
}
