






fn main() {
   let mut fahrenheit: f64 = 6.0;

   let celsius_factor: f64 = 5.0 / 9.0;

   let celsius = (fahrenheit - 32.0) * celsius_factor;

   println!("fahrenheit: {:.2}ºF", fahrenheit);  
   println!("celsius: {:.2} ºC", celsius);  
}

