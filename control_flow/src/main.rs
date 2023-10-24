fn main() {
    if_expression();
    if_else_if();
    if_in_let();
    // repeat();
    repeat_return();
    while_loop();
    while_each();
    for_each();
    for_index();
}

fn if_expression() {
    let number = 3;

    if number % 2 == 0 {
        println!("{number}는 짝수");
    } else {
        println!("{number}는 홀수");
    }
}

fn if_else_if() {
    let number = 5;

    if number % 3 == 0 {
        println!("3으로 나누어떨어집니다");
    } else if number % 2 == 0 {
        println!("2로 나누어떨어집니다");
    } else {
        println!("2나 3으로 나눠떨어지지 않습니다");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number => {number}");
}

/* loop 블록으로 반복할 수 있습니다. */
fn repeat() {
    loop {
        println!("반복!");
    }
}

/* loop 블록으로 반복할 수 있습니다. break로 중단합니다. */
fn repeat_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2;
        }
    };

    println!("최종 결괏값 => {result}");
}

fn while_loop() {
    let mut number = 0;

    while number != 5 {
        println!("{number}!");

        number += 1;
    }

    println!("완료!");
}

fn while_each() {
    let a = [2, 4, 6, 8, 10];
    let mut idx = 0;

    while idx < a.len() {
        println!("배열 값: {}", a[idx]);

        idx += 1;
    }
}

/* for구문으로 여러 값을 순차로 다룹니다. */
fn for_each() {
    let a = [2, 4, 6, 8, 10];

    for elem in a {
        println!("배열 값: {}", elem);
    }
}

fn for_index() {
    for number in (1..5) {
        println!("for {number}!");
    }
    println!("완료!");
}
