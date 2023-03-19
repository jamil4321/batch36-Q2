use scanrs::scann;
fn main() {
    // println!("Please First number");
    // let first_num: i32 = scann();
    // println!("Please Second number");
    // let second_num: i32 = scann();
    // println!(
    //     "Sum of {} and {} is {}",
    //     first_num,
    //     second_num,
    //     sum(first_num, second_num)
    // );

    println!("Please  Enter The User Name");
    let user_name: String = scann();
    if user_name == "admin" {
        println!("Login Success")
    }else{
        println!("Wrong user name")
    }

    
}
// fn sum(num_1: i32, num_2: i32) -> i32 {
//     num_1 + num_2
// }
