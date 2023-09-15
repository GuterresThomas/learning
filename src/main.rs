enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 25,
        Coin::Quarter => 50,
    }
}

fn main () {

}