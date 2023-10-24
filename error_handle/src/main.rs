fn main() {
    // vector_access();
    // open_file();
    // open_file_unwrap();
    // open_file_expect();
    let username = read_username_long();
    println!("username = {:?}", username);
}

/* panic!은 에러 메시지를 콘솔에 보이고 종료합니다. */
fn force_panic() {
    panic!("강제 종료!");
}

/* Vec에 잘못된 인덱스로 접근하면 패닉됩니다. */
fn vector_access() {
    let v = vec![1, 2, 3];

    v[100];
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;

fn open_file() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("파일 열기 실패: {:?}", error),
    };
}

use std::io::ErrorKind;

/* 에러 종류에 따른 추가 처리를 해봅시다. */
fn open_file_kind() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("파일 생성 실패: {:?}", e),
            },
            other_error => {
                panic!("파일 열기 실패: {:?}", other_error);
            }
        },
    };
}

/* unwrap은 정상값이 아닌 경우 panic!합니다. */
fn open_file_unwrap() {
    let file = File::open("hello.txt").unwrap();
}

/* expect로 쓰면 비정상값에 panic할 때 메시지를 지정할 수 있습니다. */
fn open_file_expect() {
    let file = File::open("hello.txt").expect("hello.txt 파일이 있어야 합니다.");
}

use std::io::Error;
use std::io::Read;

/* 에러 전파를 일일이 다루었을 경우 코드가 길어집니다. */
fn read_username_long() -> Result<String, Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/* ?축약 표현을 쓰면 간결하게 연쇄적인 에러를 편히 다룰 수 있습니다. */
fn read_username_short() -> Result<String, Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

/* ?축약표현을 연이어 쓸 수도 있습니다. */
fn read_username_shorter() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;

/* fs::read_to_string은 내부적으로 ?축약표현이 이미 들어있습니다. */
fn read_username_even_shorter() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}
