// use std::io;
fn main() {
    // loop infinite  forcefully break
    // For Loop range
    // let arry_number = [100, 23, 13213, 13323, 2132, 2323, 2323, 232, 32];
    // for number in arry_number {
    //     // println!("{}!", number);
    //     if number <=100 {
    //         println!("{}!", number);
    //     }
    // }

    // let name: &str = "Jamil Ahmed";

    // for char_value in name.chars() {
    //     println!("Charector {}", char_value)
    // }

    //  for keyword
    // number varibale
    // in keyword
    // range

    // While loop condition
    let mut Math_Number: usize =0;
    loop {
        let mut Math_number_input = String::new();
        println!("Enter Your Number ");
        io::stdin().read_line(&mut Math_number_input);
        Math_Number = Math_number_input.trim().parse().unwrap();
        if Math_Number <= 100 {
            break;
        } else {
            println!("Number must be less then 100")
        }
    }
    println!("Marks of Math Subject, {}", Math_Number);
}


 // let mut x = 0;
    // let mut y = 0;
    // let mut z = 0;
    // loop{
    //     println!("Value of x {x}");
    //     break;
    // }
    // while x <10 {
    //     println!("Value of x {x}");
    //     if x ==5{
    //         break;
    //     }
    //     x+=1;
    // }
    // 'firstLoop: loop {
    //     println!("value of x {x}");
    //     x += 1;
    //     'secondLoop: loop {
    //         println!("value of y {y}");
    //         y += 1;
    //         if x == 3 {
    //             y = 0;
    //             continue 'firstLoop;
    //         }
    //         if y == 10 {
    //             y = 0;
    //             break 'secondLoop;
    //         }
    //     }
    //     if x== 5{
    //         break 'firstLoop;
    //     }
    // }

    // let mut arr =[];

    // let mut arr: [i32; 5] = [0, 0, 0, 0, 0];

    // for i in (1..5) {
    //     arr[i] = i as i32;
    // }
    // println!("Value of arr {:?}", arr);
    // let mut vec:Vec<i32> = vec![0];
    // for i in 1..5 {
    //     vec[i] = i as i32;
    // }

    // println!("Value of arr {:?}",vec)
    