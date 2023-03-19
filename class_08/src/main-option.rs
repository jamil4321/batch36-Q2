// enum Option<T> {
//     None,
//     Some(T),
// }

fn main(){

    let mut null_value : Option<u8> = None;
    // println!("value of null Variable {:?}",null_value);
    null_value = Some(100);
    println!("value of null Variable {:?}",null_value);
    println!("Value of add_two_value , {:?}",sum(null_value,3));

    let value:u8 = 15;

    match value {
        99 => println!("value is less then 100"),
        98 => println!("Value is grether then 100"),
        _ => println!("value is {}",value ),
    }

    


}

fn sum(x:Option<u8>,y:u8)->u8{
    let mut return_value = y;
    return_value =  match x{
        Option::Some(a) => return_value+ a,
        Option::None => return_value,
    };
    return_value
}