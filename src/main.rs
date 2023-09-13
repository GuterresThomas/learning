
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 100;
    for i in 0..n {
        println!("{}", {fibonacci(i)});
    }
}

