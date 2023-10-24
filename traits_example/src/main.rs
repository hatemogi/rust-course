use std::fmt::Debug;
use std::fmt::Display;

trait Greet {
    fn greeting(&self) -> String;
}

#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
    Tiger,
}

impl Greet for Pet {
    fn greeting(&self) -> String {
        match self {
            Pet::Dog => String::from("멍멍"),
            Pet::Cat => String::from("야옹"),
            Pet::Tiger => String::from("어흥"),
        }
    }
}

struct Person {
    name: String,
    active: bool,
}

impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("안녕")
    }
}

fn meet(one: &impl Greet, another: &impl Greet) {
    println!("첫번째가 인사합니다 = {}", one.greeting());
    println!("두번째가 인사합니다 = {}", another.greeting());
}

fn meetBounded<T: Greet>(one: &T, another: &T) {
    println!("첫번째가 인사합니다 = {}", one.greeting());
    println!("두번째가 인사합니다 = {}", another.greeting());
}

fn multipleTraits<T: Greet + Display>(one: &T, another: &T) {
    println!("{} 인사합니다 = {}", one, one.greeting());
    println!("{} 인사합니다 = {}", another, another.greeting());
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

fn traitsWhere<T, U>(one: &T, another: &U)
where
    T: Greet + Display,
    U: Greet + Debug,
{
    println!("첫번째: {} 인사합니다 = {}", one, one.greeting());
    println!("두번째: {:?} 인사합니다 = {}", another, another.greeting());
}

fn main() {
    let cat = Pet::Cat;
    let gildong = Person {
        name: String::from("홍길동"),
        active: true,
    };
    let dooly = Person {
        name: String::from("둘리"),
        active: true,
    };
    meet(&cat, &gildong);
    meetBounded(&cat, &Pet::Dog);
    multipleTraits(&gildong, &dooly);
    traitsWhere(&gildong, &cat);
}
