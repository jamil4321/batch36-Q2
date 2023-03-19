use std::{io,collections::HashMap};
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //  let arr = [1, 2, 3, 4, 5, 6];
    //  println!("arr {:?}", arr);

    //  let mut vec: Vec<i32> = Vec::new();
    //  println!("vec {:?}", vec);

    //  for i in 0..9 {
    //      let mut Math_Number: i32;
    //      loop {
    //          let mut Math_number_input = String::new();
    //          println!("Enter Your Number ");
    //          io::stdin().read_line(&mut Math_number_input);
    //          Math_Number = Math_number_input.trim().parse().unwrap();
    //          if Math_Number <= 100 {
    //              break;
    //          } else {
    //              println!("Number must be less then 100")
    //          }
    //      }
    //      vec.push(Math_Number);
    //      println!("Vector {:?}", vec);
    //  }

    //  println!("Vector {:?} after loop end", vec);

    // let mut vec = vec![1,2,3,4,5,6,7,8,9,10];
    // println!("Vec {:?}", vec );
    // vec.pop();
    // println!("Vec {:?}", vec );
    // vec.push(11);
    // println!("Vec {:?}", vec);
    // //  let arr =[1,2,3,4,5,6,7];
    // //  println!("arr {:?}", arr[9]);
    // for  v in &mut vec{
    //    *v -=5
    // }
    // println!("Vec {:?}", vec);

    //   let row = vec![
    //       SpreadsheetCell::Int(3),
    //       SpreadsheetCell::Text(String::from("blue")),
    //       SpreadsheetCell::Float(10.12),
    //   ];

    //   println!("Vector with enum {:?}",row );
   //  let s1 = String::from("tic");
   //  let s2 = String::from("tac");
   //  let s3 = String::from("toe");

   //  let s = s1 + "-" + &s2 + "-" + &s3;
   //    println!("s {}", s );
   //  let s1 = String::from("tic");
   //  let s2 = String::from("tac");
   //  let s3 = String::from("toe");

   //  let s = format!("{s1}-{s2}-{s3}");
   //  println!("s {}", s );
   // let name = String::from("PIAIC IOT");
   // // let index = &name[0..5] ;
   

   // for c in name.chars(){
   //    println!("Chars {}",c );
   // } 

      let mut our_hash_map =  HashMap::new();
   
      println!("hash map {:?}", our_hash_map );
      our_hash_map.insert(String::from("key 1"),100);
      // our_hash_map.insert(String::from("key 1"),1000);
      our_hash_map.insert(String::from("key 2"),200);
      our_hash_map.insert(String::from("key 3"),300);
      our_hash_map.insert(String::from("key 4"),400);
      our_hash_map.entry(String::from("key 5")).or_insert(1000);

      // println!("hash map {:?}", our_hash_map );
      // println!("hash map {:?}", our_hash_map.get(&String::from("key 5")) );
      
      for (key,value) in & our_hash_map{
         println!("key value  {} {}", key ,value );
      }







}
