fn main() {
    // for loop
    for i in 1..4 { // от 1 до 4 невключительно
        // println!("Number {}", i);
    }

    for i in 1..=4 { // от 1 до 4 включительно
        // println!("Number {}", i);
    }

    for i in (1..4).rev() { // от 3 до 1
        // println!("Number {}", i);
    }

    for i in (1..4).rev().step_by(2) { // от 3 до 1 с шагом 2
        // println!("Number {}", i);
    }

    // while loop
    let mut number = 3;
    while number > 0 {
        // println!("Number {}", number);
        number -= 1;
    }

    // beak и continue
    for i in 1..10 {
        if i % 2 == 0 {
            continue;
        }
        if i > 7 {
            break;
        }
        // println!("i = {}", i);
    }

    // бесконечный цикл
    let mut number = 0;
    loop {
        number += 1;
        if (number > 3) {
            break;
        }
        // println!("number = {}", number);
    }

    // Перебор элементов массива
    let array = [1, 2, 3, 4, 5];
    for el in array {
        // println!("El = {}", el)
    }
}
