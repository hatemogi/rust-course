#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let red = Color::Red;
    let green = Color::Green;
    println!("red = {:?}", red);
    println!("red == blue? => {}", red == green);
    println!("red == red? => {}", red == Color::Red);
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}

fn message() {
    let message = Message::StartGame;
    let message2 = Message::WinPoint {
        who: String::from("길동"),
    };
    let message3 = Message::ChangePlayerName(String::from("둘리"));
}

// enum Option<T> {
//     None,
//     Some(T),
// }

fn some_values() {
    let some_number = Some(3);
    let absent_number: Option<i32> = None;

    let x: i32 = 2;

    // x + some_number;
}
