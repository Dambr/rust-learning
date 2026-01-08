use std::io;


fn main() {

    let mut num1 = "123".to_string();
    let mut num2 = "345".to_string();
    println!("Enter num1: ");
    // io::stdin().read_line(&mut num1).expect("Ошибка");
    println!("Enter num2: ");
    // io::stdin().read_line(&mut num2).expect("Ошибка");

    let data1: i16 = num1.trim().parse().expect("Enter valid number");
    let data2: u16 = num2.trim().parse().expect("Enter valid number");

    println!("data1: {}", data1 + 1);
    println!("data2: {}", data2 + 2);

    let data = data1 + data2 as i16;
    println!("data: {}", data);

    // refs

    let mut s1 = String::from("Hello");
    change(&mut s1);
    let length = get_length(&s1);
    println!("Length of '{}' is {}", s1, length);
}

fn get_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(" World");
}


