use std::fs;
use std::io;

fn main() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("{}", e),
    }
    // println!("Result: {}", result);
    let option = find_element(vec![1, 2, 3], 3);
    match option {
        Some(value) => println!("option: {}", value),
        None => println!("Error in function"),
    }

    let file_name = "output.txt";
    match write_to_file(file_name, "Hello, Rust!") {
        Ok(()) => println!("Data has been written"),
        Err(e) => println!("Error: {}", e),
    }

    match read_file(file_name) {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("b = 0"))
    } else {
        Ok(a / b)
    }
}

fn find_element(vec: Vec<i32>, value: i32) -> Option<usize> {
    vec.iter().position(|&x| x == value)
}

fn write_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(file_path, content)
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}
