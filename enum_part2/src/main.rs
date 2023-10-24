#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct RGB(u8, u8, u8);

fn color_to_rgb(color: Color) -> RGB {
    match color {
        Color::Red => RGB(255, 0, 0),
        Color::Green => RGB(0, 255, 0),
        Color::Blue => RGB(0, 0, 255),
    }
}

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName(String),
}

/* 패턴매치 기능으로 Enum 값에 따른 처리를 하기 좋습니다. */
fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("게임시작!"),
        Message::WinPoint { who } => println!("{}의 득점", who),
        Message::ChangePlayerName(_) => println!("플레이어 이름변경 요청"),
    }
}

fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let x = Some(2);
    println!("{:?}", increment(x));
    println!("{:?}", increment(None));
}
