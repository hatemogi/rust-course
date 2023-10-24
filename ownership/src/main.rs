fn main() {
    println!("Hello, world!");
    function_return_ownership();
    소유권_임대();
    string_slices();
    slice_ending_range();
}

/* 블록안에서 소유권이 유효합니다. */
fn let_scope() {
    {
        let s = "헬로";
    }
}

/* String::from으로 새 문자열을 내 소유로 만들 수 있습니다. */
fn string_from() {
    let s = String::from("헬로");
}

fn string_mutation() {
    let mut s = String::from("헬로");
    s.push_str(", 월드!");
    println!("{}", s);
}

fn simple_data_copy() {
    let x = 3;
    let y = x;
}

/* "헬로"로 부터 만든 문자열의 소유권이 s1에 있다가, s2로 넘어갔습니다. */
fn string_copy() {
    let s1 = String::from("헬로");
    let s2 = s1;
}

/* 문자열 소유권이 s2로 넘어갔기 때문에, s1은 더 이상 쓸 수 없습니다. */
fn string_copy_invalid() {
    let s1 = String::from("헬로");
    let s2 = s1;

    // println!("s1 = {}", s1);
}

/* .clone()으로 복제했기 때문에, 둘 다 소유권이 유효하고, 따라서 둘 다 쓸 수 있습니다. */
fn string_clone() {
    let s1 = String::from("헬로");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

/* 스택에 위치하는 데이터의 경우, clone없이도 복제됩니다. */
fn stack_only_copy() {
    let x = 3;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

/* 함수에 전달하는 파라미터도 소유권이 넘어갑니다. */
fn string_owner_of_function() {
    let s = String::from("헬로");
    function_transfer(s);

    let x = 3;
    function_copy(x);
}

/* s의 소유권이 함수에게로 넘어옵니다. */
fn function_transfer(s: String) {
    println!("s = {}", s);
}

/* 스택에 위치하는 데이터 타입들은 소유권이 넘어오지 않고, 복제됩니다. */
fn function_copy(x: i32) {
    println!("x = {}", x);
}

/* 함수의 반환값은, 소유권이 호출자에게 넘어갑니다. */
fn function_return_ownership() {
    let s1 = 소유권_반환();

    let s2 = String::from("헬로");

    let s3 = 소유권_가져갔다가_반환(s2);
}

fn 소유권_반환() -> String {
    let 새로운문자열 = String::from("월드");

    새로운문자열
}

fn 소유권_가져갔다가_반환(문자열: String) -> String {
    문자열
}

/* s1 소유이던 데이터의 소유권을 다시 돌려주기 위해 복잡한 작업을 하고 있습니다. */
fn 불필요한_소유권_이동() {
    let s1 = String::from("헬로");

    let (s2, len) = 문자열_길이(s1);

    println!("'{}'의 길이는 {}입니다.", s2, len);
}

fn 문자열_길이(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

/* 소유권 임대를 통해서 불필요한 소유권 전달 작업을 줄일 수 있습니다. */
fn 소유권_임대() {
    let s1 = String::from("hello");

    let len = calc_length(&s1);

    println!("문자열 '{}'의 길이는 {}.", s1, len);
}

/* 소유권 임대로 받은 s는 소유권이 넘어오지 않았으며, 따라서 반환하지 않아도 됩니다. */
fn calc_length(s: &String) -> usize {
    s.len()
}

fn 임대한_데이터_변경1() {
    let s = String::from("헬로");

    append_basic(&s);
}

/* 소유권 임대의 경우에도 mut를 명시해야 변경 가능합니다. */
fn append_basic(s: &String) {
    // s.push_str(", 월드!"); // <- 불가능
}

fn 임대한_데이터_변경2() {
    let mut s = String::from("헬로");

    append(&mut s);
}

/* 소유권 임대이지만 mut를 명시했기 때문에 .push_str로 변경 가능합니다. */
fn append(s: &mut String) {
    s.push_str(", 월드!");
}

/* mut임대는 한 번만 할 수 있습니다. */
fn 변경가능참조두번A() {
    let mut s = String::from("헬로");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

/* 두 번 한 것처럼 보이지만, r1의 소유권이 끝난 다음이므로 r2로 임대 가능합니다. */
fn 변경가능참조두번B() {
    let mut s = String::from("헬로");

    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}

/* 일반 참조 임대는 여러번 가능합니다. */
fn 기본참조에변경참조() {
    let mut s = String::from("헬로");

    let r1 = &s; // 가능
    let r2 = &s; // 가능
                 // let r3 = &mut s; // 불가능

    // println!("{}, {}, {}", r1, r2, r3);
}

fn 기본참조에변경참조가능한범위() {
    let mut s = String::from("헬로");

    let r1 = &s; // 가능
    let r2 = &s; // 가능
    println!("{}, {}", r1, r2);
    // r1과 r2가 이 이후 쓰이지 않음

    let r3 = &mut s; // 이 상황에는 가능
    println!("{}", r3);
}

fn first_word_sign(s: &String) {
    // 뭘 반환해야?
}

fn string_slices() {
    let s = String::from("헬로 월드");

    let hello = &s[0..6];
    let world = &s[7..13];

    println!("{}, {}", hello, world);
}

fn slice_starting_range() {
    let s = String::from("헬로");

    let slice = &s[0..3];
    let slice = &s[..3];
}

fn slice_ending_range() {
    let s = String::from("헬로");

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    println!("슬라이스 = {}", slice);
}

fn slice_entire() {
    let s = String::from("헬로");

    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word_reference(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_literal_slice() {
    let s: &str = "헬로";
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn parameter_as_slice() {
    let my_string = String::from("헬로 월드");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
