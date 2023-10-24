fn main() {
    println!("Hello, world!");
    let width = 20;
    let height = 30;

    println!("해당 사각형의 면적은 {}.", area0(width, height));
    show();
    dbg_macro();
}

fn area0(width: u32, height: u32) -> u32 {
    width * height
}

fn rect_as_tuple() {
    let rect = (20, 30);

    println!("해당 사각형의 면적은 {}.", area_tuple(rect));
}

fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rect_as_struct() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("해당 사각형의 면적은 {}.", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn show() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("해당 사각형은 {:?}.", rect);
}

fn dbg_macro() {
    let scale = 3;
    let rect = Rectangle {
        width: dbg!(20 * scale),
        height: 30 * scale,
    };

    dbg!(&rect);
}
