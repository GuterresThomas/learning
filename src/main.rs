struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 5.0}; //tem que ser o mesmo tipo
}