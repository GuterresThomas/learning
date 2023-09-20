/* struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 5.0}; //tem que ser o mesmo tipo
} */

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 1.2, y:3.4};
    let both_different = Point {x: 1.3, y: 5}; //agora funciona 
}


enum Result<T, E> {
    Ok(T),
    Err(E),
}