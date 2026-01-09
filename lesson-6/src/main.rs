mod math;

macro_rules! my_print {
    ($msg:expr) => {
        println!("{}", $msg);
    }
}

fn main() {
    test();
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("{} + {} = {}", a, b, c);

    let mut user_name = String::from("Max");
    greet_user(&user_name);
    change_user(&mut user_name);
    println!("User name = {}", user_name);

    let nums = (5, 6);
    let mult_result = mult(&nums);
    println!("mult {}, {} = {}", nums.0, nums.1, mult_result);
    my_print!("Hello from my print");

    let a = 10;
    let b = 5;
    let c = math::add(a, b);
    println!("{} + {} = {}", a, b, c);

    let c = math::minus(a, b);
    println!("{} - {} = {}", a, b, c);
}

fn test() {
    println!("Hello, world!");
}

fn add(a: u8, b: u8) -> u8 {
    let c = a + b;
    return c;
}

fn greet_user(name: &str) {
    println!("Hello, {}", name);
}

fn change_user(name: &mut String) {
    *name = String::from("Bob");
}

fn mult(data: &(i32, i32)) -> i32 {
    let data_0 = data.0;
    let data_1 = data.1;
    let result = data_0 * data_1;
    return result;
}
