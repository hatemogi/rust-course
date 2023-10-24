use std::io;

fn main() {
    println!("[가위, 바위, 보] 중 하나를 입력해주세요!");

    let mut decision = String::new();

    io::stdin().read_line(&mut decision).expect("입력실패");

    println!("당신의 선택: {decision}");
}
