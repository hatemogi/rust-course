fn main() {
    short_lifetime();
    ok_lifetime();
    str_lifetime();
    str_lifetime_still_ok();
    lifetime_in_struct();
}

/* 참조 수명이 원래 데이터보다 더 긴 경우 -> 컴파일 오류 */
fn short_lifetime() {
    // let x;

    // {
    //     let y = 5;
    //     x = &y;
    // }

    // println!("x = {}", x);
}

fn ok_lifetime() {
    let y = 5;
    let x = &y;

    println!("x = {}", x);
}

fn str_lifetime() {
    let s1 = String::from("가나다");
    let s2 = "하나둘셋";

    let res = longest(s1.as_str(), s2);
    println!("더 긴 문자열은 {}", res);
}

/* 참조로 반환받은 res의 수명이 s1, s2 둘 중 더 수명이 짧은 s2의 수명보다
 * 짧거나 같기 때문에 문제 없이 컴파일.
 */
fn str_lifetime_still_ok() {
    let s1 = String::from("가나다");
    {
        let s2 = "하나둘셋";

        let res = longest(s1.as_str(), s2);
        println!("이번에도 더 긴 문자열은 {}", res);
    }
}

/* res의 수명이 s2보다 길기 때문에 컴파일 오류
 */
fn str_lifetime_invalid() {
    // let s1 = String::from("가나다라마바사");
    // let res;
    // {
    //     let s2 = String::from("하나둘셋");
    //     res = longest(s1.as_str(), s2.as_str());
    // }
    // println!("더 긴 문자열은 {}", res);
}

/* 수명 표기가 필수인 경우
 */
// fn longest_without_lifetime(s1: &str, s2: &str) -> &str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }

/* 수명 표기를 잘 한 경우 */
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/* 구조체에도 수명 표기를 합니다. */
struct ImportantPart<'a> {
    part: &'a str,
}

/* 구조체 ImportantPart의 part라는 문자열 슬라이스의 수명과 같게 유지합니다. */
fn lifetime_in_struct() {
    let sentences = String::from("안녕하세요. 러스트의 참조 수명에 대해 알아볼게요.");
    let first_sentence = sentences
        .split('.')
        .next()
        .expect("마침표 '.'를 찾을 수 없어요");
    let i = ImportantPart {
        part: first_sentence,
    };
}

/* 메소드의 수명 표기는 생략규칙 3번에 따라 생략가능한 경우가 많습니다. */
impl<'a> ImportantPart<'a> {
    fn notice(&self, text: &str) -> &str {
        println!("공지사항: {}", text);
        self.part
    }
}

/* 'static 수명은 프로그램 실행 내내 유지됩니다. */
fn static_lifetime() {
    let s: &'static str = "프로그램 실행 중 내내 유효한 수명";
}
