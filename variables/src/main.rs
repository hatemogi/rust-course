#![allow(dead_code, unused_variables)]

fn main() {
    let mut x = 3;
    println!("x의 값은 {x}입니다");
    x = 7;
    println!("x의 값은 {x}입니다");

    constant();
}

const PI: f32 = 3.141592;
fn constant() {
    println!("PI상수값은 {PI}입니다.");
}

fn shadowing() {
    let x = 3;
    println!("x의 값은 {x}입니다");
    let x = x + 1;
    println!("x의 값은 {x}입니다");
    {
        let x = x * 2;
        println!("안쪽 범위에서 x의 값은 {x}입니다");
    }
    println!("x의 값은 {x}입니다");
}
