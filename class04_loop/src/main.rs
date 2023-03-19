use std::io;

use cumstom_lib::addtion;
fn main() {
    let mut check = true;
    let mut Math_Number: char = 'a';


    while check  {

        let mut Math_number_input = String::new();
        println!("Enter Your Number ");

        io::stdin()
            .read_line(&mut Math_number_input)
            .expect("IO Failure");



        Math_Number = match Math_number_input.trim().parse() {

            Ok(num) => {
                if num == '+' || num == '-' || num == '*' || num == '/' {
                    check = false;
                    num
                } else {
                    check = true;
                    'a'
                }
            }

            
            Err(_) => {
                println!("Character required");
                check = true;
                'a' as char
            }
        };
    }
    println!("Value of Input {}", Math_Number);
}
