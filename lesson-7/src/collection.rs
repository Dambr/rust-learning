use std::{collections::HashMap};

pub fn with_collections() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 5);
    scores.insert("Grey", 30);

    println!("Hash: {:?}", scores);

    scores.insert("Red", 15);
    println!("Red Hash: {}", scores.get("Red").unwrap());

    scores.remove("Red");
}