pub fn with_strings() {
    // Строки
    let s1 = String::new();
    let s2 = String::from("Hello");
    println!("s1 {}", s1);
    println!("s2 {}", s2);

    let s3 = s1 + &s2; // После этого s1 перестанет быть доступной

    let mut word: String = String::new();
    word.push_str("Hello");
    word.push(' ');
    word.push_str("Word");
    println!("word = {}", word);
}