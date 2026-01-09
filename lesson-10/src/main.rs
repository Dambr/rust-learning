
struct Point {
    x: f64,
    y: f64,
    z: f64
}

struct Color(u8, u8, u8);

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

struct Pair <T> {
    first: T,
    second: T
}

fn main() {

    let point = Point { x: 3.0, y: 2.0, z: 0.0 };
    let color = Color(255, 255, 255);
    let rectangle = Rectangle{width: 2.0, height: 3.0};

    println!("point: ({}, {}, {})", point.x, point.y, point.z);

    println!("color: ({}, {}, {})", color.0, color.1, color.2);

    println!("area: {}", rectangle.area());
    println!("perimeter: {}", rectangle.perimeter());

    let moving = Direction::Up;
    move_player(moving);

    let pair = Pair {first: 10, second: 20};
    println!("Pair: {}, {}", pair.first, pair.second);
}

fn move_player(moving: Direction) {
    match moving {
        Direction::Left => println!("move left"),
        Direction::Right => println!("move right"),
        Direction::Up => println!("move up"),
        Direction::Down => println!("move down"),
    };
}
