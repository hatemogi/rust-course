fn smallest_i32(list: &[i32]) -> &i32 {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn smallest_char(list: &[char]) -> &char {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn duplicated_functions() {
    let numbers = vec![3, 4, 1, 6, 8, 10];

    let result = smallest_i32(&numbers);
    println!("가장 작은 수는 {}", result);

    let chars = vec!['홍', '길', '동'];

    let result = smallest_char(&chars);
    println!("가장 작은 글자는 {}", result);
}

/* 어떤 타입 T를 받을 건데, 그 T는 PartialOrd 트레이트를 구현해야 합니다.
 * PartialOrd트레이트를 구현했기 때문에 < 비교를 할 수 있습니다.
 */
fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn using_generics() {
    let numbers = vec![3, 4, 1, 6, 8, 10];

    let result = smallest(&numbers);
    println!("가장 작은 수는 {}", result);

    let chars = vec!['홍', '길', '동'];

    let result = smallest(&chars);
    println!("가장 작은 글자는 {}", result);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn generic_points() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 5.0, y: 1.0 };
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn method_generic() {
    let p1 = Point { x: 1, y: 2 };
    println!("p1.x() = {}", p1.x());
}

fn main() {
    duplicated_functions();
    using_generics();
    generic_points();
    method_generic();
}
