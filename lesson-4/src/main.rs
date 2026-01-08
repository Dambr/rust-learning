fn main() {
    // if .. else
    
    let number = 10;
    let is_has_car = true;
    if number > 5 && is_has_car {
        println!("number > 5");
    } else if number == 5 {
        println!("number = 5");
    } else {
        println!("else block");
    }

    // Тернарны оператор
    let condition = false;
    let number = if condition { 5 } else { 10 };
    println!("Number {}", number);

    // Match

    let number = 3;
    match number {
        1 => println!("number is {}", number),
        2 => println!("number is {}", number),
        3 => {
            println!("number is {}", number);
            println!("this is code subblock");
        },
        4 => println!("number is {}", number),
        _ => println!("number is unknown"), // default
    }

}
