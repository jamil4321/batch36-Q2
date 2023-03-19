// fn main (){
//     let num_1:Option<i32> = Some(100);
//     let num_2:Option<i32>  = Some(5);

//     let mut student_name:Option<&str> = None;
//     student_name =Some("Jamil Ahmed");
//     println!("{:?}",student_name);

//     match student_name{
//         Option::Some(name)=>println!("{}",name ),
//         Option::None =>println!("Value is null")
//     }
//     match num_1 {
//         Option::Some(first_value) => match  num_2{
//             Option::Some(second_value) => 
//             println!("sum of 2 value is {} " , first_value+second_value),
//             Option::None=>println!("seond Varible is null")
//         },
//         Option::None=> println!("first varible is null")

//     }
// }

// #[derive(Debug)]
// enum Option_for_number {
//     intiger_value(u8,i32,String),
//     Null
// }


// fn main(){
//     let mut num_1 = Option_for_number::intiger_value(200,2999,String::from("Value"));
//     println!("Custom Option Enum {:?}", num_1 );
//         match num_1{
//         Option_for_number::intiger_value(num,num_2,string_value)=>println!("value of intiger {} {} {}",num ,num_2,string_value),
//         Option_for_number::Null => println!("value is null")
//         }
    
// }




fn main (){
        let num_1:Option<i32> = Some(100);
        // let num_2:Option<i32>  = Some(5);

        if let Some(num)= num_1{
            println!("print Some Value {}", num );
        }
    
}