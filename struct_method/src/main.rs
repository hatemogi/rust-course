#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("이 사각형의 면적은 {}입니다.", rect.area());
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn associated_fn() {
    println!("정사각형 = {:?}", Rectangle::square(30));
}
