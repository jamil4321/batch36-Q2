// fn main() {

//     let mut s1 = String::from("Hello");
//     // pointer reffrence
//     // 
//     greating(&mut s1);

//     greating(&mut s1);
//     // greating(s1);
//     // greating(s1);
//     // greating(s1);
//     println!("{}",s1);
// } 

// fn greating(value:&mut String){
//     value.push_str(", World");
//     println!("{}", value );
// }


// fn main() {
//     let mut s1 = String::from("hello");
//     let a = &mut s1;
//     // println!("{:?}");
//     a.push_str(", world");
//     let b = &mut s1;
//     println!("{}",b);

//     // {
//     //     let a=&s1;
//     // }

//     // // println!("{}", a);
//     // {
//     //     let b = &mut s1;
//     // }
   
//     // let a = &s1;
//     // println!("{}", a);
//     // let len = calculate_length(&s1);

   
// }

// // fn calculate_length(s:& String) -> usize {
// //     s.len()
// // }


// fn main (){
//     // let a = greating();

//     // println!("{}",a);


//     let mut greating:&str = "Hello";

//     greating.push_str(", World")

// }



// fn greating()->&String{
//     // s ;;; 
//     let s = String::from("Hello, World") 
   
//     &s
// }//  rust.drop(s)

#[derive(Debug)]
struct User{
    name:String,
    email:String,
    gender:String,
    age:i8,
}


fn main(){
    // let mut user1:User= User {
    //     name:String::from("Jamil"),
    //     email:String::from("jami@email.com"),
    //     gender:String::from("Male"),
    //     age:30
    // };

    // let new_user = create_new_user(
    //     String::from("Fahim"),
    //     String::from("fahim@email.com"),
    //     String::from("Male"),
    //     24 
    // );
    // let new_user2 = create_new_user(
    //     String::from("Abdul Rasheed"),
    //     String::from("abc@email.com"),
    //     String::from("Male"),
    //     24 
    // );


let name=String::from("Jamil");
let gender=String::from("M");
let email=String::from("jami@demo.com");
let age = 25;
    
    let struc_user = create_new_user(gender,email,age,name);
    println!("Struct {:?}",  struc_user);
    let new_struct_user = User {
        name:String::from("Adil"),
        email:String::from("abc@gmail.com"),
        ..struc_user
    };
   
    println!("Struct {:?}",  new_struct_user);
}


fn create_new_user( gender:String,email:String,age:i8,name:String)->User{
    User{name,email,gender,age}
}