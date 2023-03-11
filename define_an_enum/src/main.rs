#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let direction = Direction::Up;
    value_directions(direction);

    let coin = Coin::Dime;
    value_in_cents(coin);
}

fn value_directions(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }
}

fn value_in_cents(coin: Coin) {
    match coin {
        Coin::Penny => println!("{}", 1),
        Coin::Nickel => println!("{}", 5),
        Coin::Dime => println!("{}", 10),
        Coin::Quarter => println!("{}", 25),
    }
}