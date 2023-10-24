#![allow(dead_code, unused_variables)]

fn main() {
    println!("Hello, world!");
    numeric_calculation();
    tuple_destructuring();
    tuple_access();
}

fn floating_points() {
    let x = 3.14; // 실수 리터럴의 기본은 f64 타입입니다.
    let y: f32 = 3.14; // f32로 명시할 수 있습니다.
}

fn numeric_calculation() {
    // 더하기
    let add = 3 + 8;

    // 빼기
    let sub = 26.5 - 2.1;

    // 곱하기
    let mul = 7 * 20;

    // 나누기
    let quotient = 12.0 / 3.14;
    let truncated = 7 / 5; // 결과는 1

    // 나머지
    let remainder = 46 % 5;

    println!("3 + 8 = {add}");
    println!("26.5 - 2.1 = {sub}");
    println!("7 * 20 = {mul}");
    println!("12.0 / 3.14 = {quotient}");
    println!("7 / 5 = {truncated}");
    println!("46 % 5 = {remainder}");
}

fn boolean_type() {
    let t = true;

    let f: bool = false;
}

/* char타입으로 유니코드 한글자를 다룹니다. */
fn character_type() {
    let c = 'A';
    let z: char = '가';
    let unicorn = '🦄';
}

/* 튜플 타입은 고정된 크기의 여러 타입을 묶어서 하나의 타입으로 씁니다. */
fn tuple_type() {
    let t: (i32, bool, f64) = (32, true, 1.41);
}

/* 튜플을 구조분해해서 안에 있는 요소를 편히 꺼내 쓸 수 있습니다. */
fn tuple_destructuring() {
    let t = (32, true, 1.41);

    let (x, y, z) = t;

    println!("z의 값은 {z}입니다");
}

/* 튜플의 원소 순서를 기준으로 접근하기도 합니다. */
fn tuple_access() {
    let t = (32, true, 1.41);

    let first = t.0;
    let second = t.1;
    let last = t.2;

    println!("({first}, {second}, {last}");
}

fn array_type() {
    let xs = [1, 2, 3, 4, 5];
}

/* Rust에서 배열은 "고정된" 크기를 다룹니다. */
fn array_data() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let sevens = [7; 5];
}

fn array_access() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
