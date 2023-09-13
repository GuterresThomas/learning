
fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    } 

    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}

fn main() {
    let n = 50;
    for i in 0..n {
        println!("{}", {fibonacci(i)});
    }
}

