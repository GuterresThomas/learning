
fn main () {
    let mut count = 0;
    
    if let Coin::Quarter (state) = coin {
        println!("State Quarter from {}", state);
    }else {
        count += 1;
    }
}