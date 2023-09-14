fn main () {
    enum Option <T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    let some_string = Some(String::from("hello"));

}
