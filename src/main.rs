fn main () {
   let s1 = String::from("tic");
   let s2: String = String::from("tac");
   let s3: String = String::from("toe");

   let s = format!("{}-{}-{}", s1, s2, s3);
   println!("{}", s)
}