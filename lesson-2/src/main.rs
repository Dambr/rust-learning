fn main() {
    // const
    const MAX_VALUE: u8 = 10;
    println!("MAX_VALUE: {}", MAX_VALUE);

    // Tuple
    let mut user: (u8, bool, char, f32) = (123, true, 'r', 4.5);
    user.0 = 22;

    println!("user.id: {}", user.0);
    println!("user.male: {}", user.1);
    println!("user.rating: {}", user.2);
    println!("user.mean: {}", user.3);

    // Array
    let mut nums: [i8; 4] = [1, 2, 3, 4];
    nums[0] = 10;
    println!("nums[0]: {}", nums[3]);
}
