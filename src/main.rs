use std::collections::HashMap;

fn main () {
   let mut scores = HashMap::new();

   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("yellow"), 50);//k = key e v = value


   for (key,value) in &scores {
    println!("{key}: {value}");
   }

}