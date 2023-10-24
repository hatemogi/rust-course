fn main() {
    println!("헬로, 월드!");

    a_function();

    print_number(3);
    add(3, 2);
    let y = multiply(3, 2);
    println!("y = 3 * 2 = {y}");
}

fn a_function() {
    println!("다른 함수입니다.")
}

fn print_number(x: i32) {
    println!("x의 값은 {x}입니다.")
}

fn add(a: i32, b: i32) {
    let sum = a + b;
    println!("a + b = {sum}");
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

const PI: f64 = 3.141592;

fn circle_area(r: f64) -> f64 {
    PI * r * r
}
